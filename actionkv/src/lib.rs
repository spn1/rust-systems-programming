/// A recreation of a key-value database store.
/// This library file denotes the writing of the data to files

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, SeekFrom };
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc::crc32;
use serde::{Deserialize, Serialize};

type ByteString = Vec<u8>;
type ByteStr = [u8];

/// BitCask file format
/// checksum  key_len  val_len    key       val
///  [ | | ]  [ | | ]  [ | | ]  [........][.........]
///  3 bytes  3 bytes  3 bytes   ..variable bytes..  

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

#[derive(Debug)]
pub struct ActionKV {
    f: File,
    pub index: HashMap<ByteString, u64>
}

impl ActionKV {
    /// Opens (or creates) the file at the specified path to be read from, and
    /// initializes the index
    pub fn open (path: &Path) -> io::Result<Self> {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path)?;
        let index = HashMap::new();
        Ok(ActionKV { f, index })
    }

    /// Loads an entry from the file wherever it happens to be reading
    /// from the file at that point in time.
    fn process_record<R: Read>( f: &mut R ) -> io::Result<KeyValuePair> {
        // Key / Value entry starts with Checksum
        // Then 4 bytes defining the length of the key
        // Then 4 bytes defining the length of the value
        let saved_checksum = f.read_u32::<LittleEndian>()?;
        let key_len = f.read_u32::<LittleEndian>()?;
        let val_len = f.read_u32::<LittleEndian>()?;

        // Therefore, the key/value path has the length key_len + val_len
        let data_len = key_len + val_len;

        // Read that much data from the file
        let mut data = ByteString::with_capacity(data_len as usize);

        // This section is put in a block because .take(..) creates
        // a new Read insteance
        {
            // Using the opened file stream, 
            f.by_ref()
                .take(data_len as u64)
                .read_to_end(&mut data)?;
        }

        debug_assert_eq!(data.len(), data_len as usize);

        // Check that the data isn't corrupted
        let checksum = crc32::checksum_ieee(&data);
        if checksum != saved_checksum {
            panic!(
                "Data corrupted: {:08x} != {:08x}",
                checksum, saved_checksum
            )
        }

        // vec.split_off removes a subslice of the given range (key_len)
        // from the vector and returns it. 
        let value = data.split_off(key_len as usize);
        let key = data;

        Ok(KeyValuePair { key, value })
    }

    /// Set the file cursor to be at the end of the file
    pub fn seek_to_end(&mut self) -> io::Result<u64> {
        // SeekFrom::End(0) starts the "seeking" 0 bytes from the end of the file
        self.f.seek(SeekFrom::End(0))
    }

    /// Load the file into the HashMap
    pub fn load(&mut self) -> io::Result<()> {
        let mut f = BufReader::new(&mut self.f);

        loop {
            // SeekFrom::Current(0) moves the cursor 0 bytes from it's current location
            // So it doesn't move it at all and allows us to get the current position
            let current_position = f.seek(SeekFrom::Current(0))?;

            let maybe_kv = ActionKV::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => {
                    match err.kind() {
                        io::ErrorKind::UnexpectedEof => {
                            break;
                        },
                        _ => return Err(err)
                    }
                }
            };

            self.index.insert(kv.key, current_position);
        }

        Ok(())
    }

    /// Gets the specified key from the HashMap index
    pub fn get(
        &mut self,
        key: &ByteStr
    ) -> io::Result<Option<ByteString>> {
        let position = match self.index.get(key) {
            None => return Ok(None),
            Some(position) => *position
        };

        let kv = self.get_at(position)?;

        Ok(Some(kv.value))
    }

    /// Gets the data from the specified position in the database
    pub fn get_at(
        &mut self,
        position: u64
    ) -> io::Result<KeyValuePair> {
        let mut f = BufReader::new(&mut self.f);
        // Set the cursor to be a the position argument and start the database read
        f.seek(SeekFrom::Start(position))?;
        let kv = ActionKV::process_record(&mut f)?;

        Ok(kv)
    }

    /// Find the specified key "target" in the database
    /// Returns the position of the key and it's value
    pub fn find(
        &mut self,
        target: &ByteStr
    ) -> io::Result<Option<(u64, ByteString)>> {
        let mut f = BufReader::new(&mut self.f);

        let mut found: Option<(u64, ByteString)> = None;

        loop {
            let position = f.seek(SeekFrom::Current(0))?;

            let maybe_kv = ActionKV::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => {
                    match err.kind() {
                        io::ErrorKind::UnexpectedEof => {
                            break;
                        },
                        _ => return Err(err),
                    }
                }
            };
            
            if kv.key == target {
                found = Some((position, kv.value));
            }

            // We keep looping through to the end of the file because
            // if the key has been overwritten, it will be recorded as
            // such later in the file
        };

        Ok(found)
    }

    /// Inserts a key/value pair into the database
    /// Also inserts it into the index hashmap
    pub fn insert(
        &mut self,
        key: &ByteStr,
        value: &ByteStr
    ) -> io::Result<()> {
        let position = self.insert_but_ignore_index(key, value)?;

        self.index.insert(key.to_vec(), position);
        Ok(())
    }

    /// Inserts a key/value pair into the database
    pub fn insert_but_ignore_index(
        &mut self,
        key: &ByteStr,
        value: &ByteStr
    ) -> io::Result<u64> {
        let mut f = BufWriter::new(&mut self.f);

        let key_len = key.len();
        let val_len = value.len();

        // Push bytes from key/value in temporary [u8] buffer
        let mut tmp = ByteString::with_capacity(key_len + val_len);
        for byte in key {
            tmp.push(*byte);
        }

        for byte in value {
            tmp.push(*byte);
        }

        // Calculate checksum
        let checksum = crc32::checksum_ieee(&tmp);

        // Starting from end of file, write the bytes in the BitCask format
        let next_byte = SeekFrom::End(0);
        let current_position = f.seek(SeekFrom::Current(0))?;
        f.seek(next_byte)?;
        f.write_u32::<LittleEndian>(checksum)?;
        f.write_u32::<LittleEndian>(key_len as u32)?;
        f.write_u32::<LittleEndian>(val_len as u32)?;
        f.write_all(&tmp)?;

        Ok(current_position)
    }

    #[inline]
    pub fn update(
        &mut self,
        key: &ByteStr,
        value: &ByteStr
    ) -> io::Result<()> {
        self.insert(key, value)
    }

    #[inline]
        pub fn delete(
            &mut self,
            key: &ByteStr
        ) -> io::Result<()> {
            self.insert(key, b"")
        }
}
/// A recreation of a key-value database store.
/// This file compiles to a binary that provides an interface for
/// using the database

use libactionkv::ActionKV;

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    akv_mem.exe FILE get KEY
    akv_mem.exe FILE delete KEY
    akv_mem.exe FILE update KEY VALUE
    akv_mem.exe FILE insert KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE update KEY VALUE 
    akv_mem FILE insert KEY VALUE
";

fn main() {
    // Get commandline arguments
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    // Try to open file
    let path = std::path::Path::new(&fname);
    let mut store = ActionKV::open(path).expect("unable to open file");

    // Read the data from the file in BitCask file format
    store.load().expect("unable to load data");

    // Perform operation based on arg action
    match action {
        "get"       => match store.get(key).unwrap() {
            None            => eprintln!("{:?} not found", key),
            Some(value)     => println!("{:?}", value)
        },

        "delete"    => store.delete(key).unwrap(),

        "insert"    => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap();
        },

        "update"    => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap();
        },

        _           => eprintln!("{}", &USAGE)
    }
}
/// Recreating a CHIP-8 CPU

// Operation (op) - a procedure supported natively by a system;
// Register - a container for data that the CPU reads from directly
// Opcode - A number that maps to a specific op

struct CPU {
    registers: [u8; 16],
    position_in_memory: usize, // also referred to as program_counter
    memory: [u8; 0x1000],
}

impl CPU {
    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (result, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = result;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        // opcode is u16, byes are u8. Need to convert to u8s into u16
        // Do it by shifting op_byte1 (8 most significant bits) left 8
        // Then OR that value with op_byte2 to get the bytes combined.
        // I.e. 00110011 << 8 = 00110011_00000000
        //      00110011__00000000 | 00100010 = 00110011_00100010
        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            // Represented in hex, the opcode is split into the "high byte" and the "low byte"
            // 0x8014 -> 0b80 is the high byte, 0b14 is the low byte
            // Each half-a-byte (4 bits) is called a nibble
            let opcode = self.read_opcode();

            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0)        => { return; },
                (0x8, _, _, 0x4)    => self.add_xy(x, y),
                _                   => todo!("opcode {:04x}", opcode),
            }
        }
    }
}

pub fn run() {
    let mut cpu = CPU {
        memory: [0; 4096],
        registers: [0; 16],
        position_in_memory: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;
    mem[0] = 0x80; mem[1] = 0x14;
    mem[2] = 0x80; mem[3] = 0x24;
    mem[4] = 0x80; mem[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);

    print!("Result of operation 0x8014 = {:?}", cpu.registers[0])
}
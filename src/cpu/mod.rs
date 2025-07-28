/// Recreating a CHIP-8 CPU

// Operation (op) - a procedure supported natively by a system;
// Register - a container for data that the CPU reads from directly
// Opcode - A number that maps to a specific op

struct CPU {
    current_operation: u16,
    registers: [u8; 2],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    fn run(&mut self) {
        // Represented in hex, the opcode is split into the "high byte" and the "low byte"
        // 0x8014 -> 0b80 is the high byte, 0b14 is the low byte
        // Each half-a-byte (4 bits) is called a nibble
        let opcode = self.read_opcode();

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {:004x}", opcode),
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        // Result is stored in register x
        self.registers[x as usize] += self.registers[y as usize];
    }
}

pub fn run() {
    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2]
    };

    // Each 4 bits of the operation code indicates what will happen
    //      8 = indiates the operation will involve two registers
    //      0 = refers to register 0
    //      1 = refers to register 1
    //      4 = indicates addition
    cpu.current_operation = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.run();

    print!("Result of operation 0x8014 = {:?}", cpu.registers[0])
}
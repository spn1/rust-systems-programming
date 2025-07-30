/// Recreating a CHIP-8 CPU

// Operation (op) - a procedure supported natively by a system;
// Register - a container for data that the CPU reads from directly
// Opcode - A number that maps to a specific op

struct CPU {
    registers: [u8; 16],
    position_in_memory: usize, // also referred to as program_counter
    memory: [u8; 4096],
    stack: [u16; 16], // stack will overflow after 16 function calls
    stack_pointer: usize,
}

impl CPU {
    /// Adds the value of kk to register vx
    fn add(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] += kk; 
    }

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

    /// AND of x & y
    fn and_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ & y_;
    }

    /// OR of x & y
    fn or_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ | y_;
    }

    /// XOR of x & y
    fn xor_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ ^ y_;
    }

    fn store_if_equal(&mut self, vx: u8, kk: u8) {
        if vx == kk {
            self.position_in_memory += 2;
        }
    }

    fn store_if_not_equal(&mut self, vx: u8, kk: u8) {
        if vx != kk {
            self.position_in_memory += 2;
        }
    }

    /// Load value kk into register vx
    fn load(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] = kk; 
    }

    /// Moves to specific location in memory
    fn jump_to(&mut self, addr: u16) {
        self.position_in_memory = addr as usize;
    }

    /// Reads the opcode from position_in_memory
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

    /// Calls a function by moving to the memory address where the function is located.
    /// Also records the location in memory before the call on the stack so that it can
    /// return after the function call is complete
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow");
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    /// Returns to the previous position in memory after a function call
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.position_in_memory = addr as usize;
    }

    fn run(&mut self) {
        loop {
            // Represented in hex, the opcode is split into the "high byte" and the "low byte"
            // 0x8014 -> 0b80 is the high byte, 0b14 is the low byte
            // Each half-a-byte (4 bits) is called a nibble
            let opcode = self.read_opcode();

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;
            let kk = (opcode & 0x00FF) as u8;
            let addr = opcode & 0x0FFF;

            self.position_in_memory += 2;


            match (c, x, y, d) {
                (0, 0, 0, 0)        => { return; },
                (0, 0, 0xE, 0xE)    => self.ret(),
                (0x1, _, _, _)      => self.jump_to(addr),
                (0x2, _, _, _)      => self.call(addr),
                (0x3, _, _, _)      => self.store_if_equal(x, kk),
                (0x4, _, _, _)      => self.store_if_not_equal(x, kk),
                (0x5, _, _, _)      => self.store_if_not_equal(x, y),
                (0x6, _, _, _)      => self.load(x, kk),
                (0x7, _, _, _)      => self.add(x, kk),
                (0x8, _, _, _)      => {
                    match d {
                        0 => { self.load(x, self.registers[y as usize]) },
                        1 => { self.or_xy(x, y) },
                        2 => { self.and_xy(x, y) },
                        3 => { self.xor_xy(x, y) },
                        4 => { self.add_xy(x, y); },
                        _ => { todo!("opcode: {:04x}", opcode); },
                    }
                },
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }
}

pub fn run() {
    let mut cpu = CPU {
        stack: [0; 16],
        stack_pointer: 0,
        memory: [0; 4096],
        registers: [0; 16],
        position_in_memory: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    // Initial process in memory
    mem[0x000] = 0x21; mem[0x001] = 0x00; // Call (2) function at (0x100);
    mem[0x002] = 0x21; mem[0x003] = 0x00; // Call (2) function at (0x100);
    mem[0x004] = 0x00; mem[0x005] = 0x00; // END

    // Function in memory
    mem[0x100] = 0x80; mem[0x101] = 0x14; // (add registers 0 and 1)
    mem[0x102] = 0x80; mem[0x103] = 0x14; // (add registers 0 and 1)
    mem[0x104] = 0x00; mem[0x105] = 0xEE; // RETURN opcode

    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    print!("Result of operation 0x8014 = {:?}", cpu.registers[0])
}
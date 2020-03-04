//! Implementation of the CHIP-8 ADD_XY instruction with a simple processing
//! loop to perform multiple instructions.
struct CPU {
    registers: [u8; 16], // Full CHIP-8 register range
    memory: [u8; 4096],  // Whopping 4Kb memory!
    // Book calls this `position_in_memory`, renamed for consistency with
    // CPU terminology.
    program_counter: usize, // usize to simplify array indexing
    stack_pointer: usize,
    stack: [u16; 16], // Avoid more than 16 nested function calls...
}

impl CPU {
    fn run(&mut self) {
        loop {
            // Fetch instruction from memory
            // 16 bit opcodes stored as two consecutive bytes in memory
            // Interpret each byte as u16 to provide space to join together
            // into a single opcode
            let op_byte1 = self.memory[self.program_counter] as u16;
            let op_byte2 = self.memory[self.program_counter + 1] as u16;
            let opcode = op_byte1 << 8 | op_byte2;

            // Decode
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let op_minor = (opcode & 0x000F) as u8;
            let addr = opcode & 0x0FFF;

            // Point to next instruction
            self.program_counter += 2;

            match opcode {
                0x0000 => return,
                0x00EE => self.ret(),
                0x2000..=0x2FFF => self.call(addr),
                0x8000..=0x8FFF => match op_minor {
                    4 => self.add_xy(x, y),
                    _ => unimplemented!("opcode {:04x}", opcode),
                },
                _ => unimplemented!("opcode {:04x}", opcode),
            }
        }
    }

    /// Perform a function call - saves current program counter on the stack
    /// and jumps to `addr`.
    fn call(&mut self, addr: u16) {
        if self.stack_pointer > self.stack.len() {
            panic!("Stack overflow!")
        }

        // Save current program counter (pointing to *next* instruction due to
        // increment in `self.run` loop)
        self.stack[self.stack_pointer] = self.program_counter as u16;
        self.stack_pointer += 1;
        // Jump to function address
        self.program_counter = addr as usize;
    }

    /// Return from a function call - restores previous program counter from
    /// the stack.
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow!");
        }

        // Jump to call site
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer] as usize;
    }

    /// Adds the value stored in register `y` to the value stored in register
    /// `x`.
    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

fn main() {
    // Set up the CPU to perform 5 + (10 * 2) + (10 * 2) with function calls
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        program_counter: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    // 0x2100 - "call the function at 0x100"
    cpu.memory[0x000] = 0x21;
    cpu.memory[0x001] = 0x00;
    // 0x2100 - "call the function at 0x100"
    cpu.memory[0x002] = 0x21;
    cpu.memory[0x003] = 0x00;

    // 0x8014 - "add register 1's value to register 0"
    cpu.memory[0x100] = 0x80;
    cpu.memory[0x101] = 0x14;
    // 0x8014 - "add register 1's value to register 0"
    cpu.memory[0x102] = 0x80;
    cpu.memory[0x103] = 0x14;
    // 0x00EE - "return from function"
    cpu.memory[0x104] = 0x00;
    cpu.memory[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}

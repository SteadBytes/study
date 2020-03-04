//! Implementation of the CHIP-8 ADD_XY instruction with a simple processing
//! loop to perform multiple instructions.
const ARITHMETIC_AND_LOGIC: u8 = 0x8;
const HALT: u8 = 0x0; // 0x0000 -> halt CPU loop (not part of CHIP-8 spec)
const ADD_XY: u8 = 0x4;

struct CPU {
    registers: [u8; 16], // Full CHIP-8 register range
    // Book calls this `position_in_memory`, renamed for consistency with
    // CPU terminology.
    program_counter: usize, // usize to simplify array indexing
    memory: [u8; 4096],     // Whopping 4Kb memory!
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
            let raw_op = op_byte1 << 8 | op_byte2;
            // Decode
            let op_major = ((raw_op & 0xF000) >> 12) as u8;
            let x = ((raw_op & 0x0F00) >> 8) as u8;
            let y = ((raw_op & 0x00F0) >> 4) as u8;
            let op_minor = (raw_op & 0x000F) as u8;

            // Point to next instruction
            self.program_counter += 2;

            match (op_major, op_minor) {
                (HALT, HALT) => {
                    return;
                }
                (ARITHMETIC_AND_LOGIC, ADD_XY) => {
                    self.add_xy(x, y);
                }
                _ => unimplemented!("opcode {:04x}", raw_op),
            }
        }
    }

    /// Adds the value stored in register `y` to the value stored in register
    /// `x`.
    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

fn main() {
    // Set up CPU to perform 5 + 10 + 10 + 10
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        program_counter: 0,
    };

    // Initialise registers with values to add
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    // Load opcode 0x8014 - "add value in register 1 to register 0" = (5 + 10)
    cpu.memory[0] = 0x80;
    cpu.memory[1] = 0x14;
    // Load opcode 0x8024 - "add value in register 2 to register 0" = (15 + 10)
    cpu.memory[2] = 0x80;
    cpu.memory[3] = 0x24;
    // Load opcode 0x8034 - "add value in register 3 to register 0" (25 + 10)
    cpu.memory[4] = 0x80;
    cpu.memory[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}

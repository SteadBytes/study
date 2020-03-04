//! Minimal implementation of the CHIP-8 ADD_XY instruction.
const ADD_XY: u8 = 0x8;

struct CPU {
    current_operation: u16,
    registers: [u8; 2], // ADD_XY needs only 2 registers
}

impl CPU {
    fn run(&mut self) {
        let encoded_op = self.current_operation;
        // Decode into operation and operands
        // Other components not decoded as only ADD_XY implemented here
        let op = ((encoded_op & 0xF000) >> 12) as u8;
        let x = ((encoded_op & 0x0F00) >> 8) as u8;
        let y = ((encoded_op & 0x00F0) >> 4) as u8;

        match op {
            ADD_XY => {
                self.add_xy(x, y);
            }
            _ => unimplemented!(),
        }
    }

    /// Adds the value stored in register `y` to the value stored in register
    /// `x`.
    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

fn main() {
    // Set up CPU to perform 5 + 10 in registers 0 and 1
    let mut cpu = CPU {
        current_operation: 0x8014,
        registers: [0; 2],
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.run();

    assert_eq!(cpu.registers[0], 15);

    println!("5 + 10 = {}", cpu.registers[0]);
}

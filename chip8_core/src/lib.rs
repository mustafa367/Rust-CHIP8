pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096; // bytes
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const START_ADDR: u16 = 0x200;

const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
0x20, 0x60, 0x20, 0x20, 0x70, // 1
0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
0x90, 0x90, 0xF0, 0x10, 0x10, // 4
0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
0xF0, 0x10, 0x20, 0x40, 0x40, // 7
0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
0xF0, 0x90, 0xF0, 0x90, 0x90, // A
0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
0xF0, 0x80, 0x80, 0x80, 0xF0, // C
0xE0, 0x90, 0x90, 0x90, 0xE0, // D
0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];

pub struct Emu {
    pc: u16, // program counter
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_HEIGHT * SCREEN_WIDTH], 
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
    sp: u16,
    keys: [bool; NUM_KEYS],
    stack: [u16; STACK_SIZE],
    dt: u8,
    st: u8,
}

impl Emu {
    pub fn new() -> Self {
        let mut new_emu = Self{
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_HEIGHT * SCREEN_WIDTH], 
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            keys: [false; NUM_KEYS],
            stack: [u16; STACK_SIZE],
            dt: 0,
            st: 0,
        }
        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        new_emu
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_HEIGHT * SCREEN_WIDTH];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        sp: 0;
        keys: [false; NUM_KEYS];
        stack: [u16; STACK_SIZE];
        dt: 0;
        st: 0;
        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize];
    }

    pub fn tick(&mut self) {
        // Fetch
        let op = self.fetch();
        // Decode & Execute
        self.execute(op);
    }

    fn execute(&mut self, op: u16) {
        let hex1 = (op & 0xF000) >> 12;
        let hex2 = (op & 0x0F00) >> 08;
        let hex3 = (op & 0x00F0) >> 04;
        let hex4 = (op & 0x000F) >> 00;

        match (hex1, hex2, hex3, hex4, ) {
            (_, _, _, _, ) => unimplemented!("Unimplemented opcode: {}", op),
        }
    }

    fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        op
    }     

    pub fn tick_timers(& mut self) {
        if self.dt > 0 {
            self.dt-= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // Beep
            }
            self.st -= 1;
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);      
//     }
// }

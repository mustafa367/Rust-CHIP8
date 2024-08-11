use rand::random;

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
            stack: [0; STACK_SIZE],
            dt: 0,
            st: 0,
        };
        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        new_emu
    }
        

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_HEIGHT * SCREEN_WIDTH];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.keys = [false; NUM_KEYS];
        self.stack = [0; STACK_SIZE];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
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
            (0x0, 0x0, 0x0, 0x0, ) => return,
            (0x0, 0x0, 0xE, 0x0, ) => {
                self.screen = [false; SCREEN_HEIGHT * SCREEN_WIDTH];
            },
            (0x0, 0x0, 0xE, 0x0, ) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            },
            (0x1,   _,   _,   _, ) => {
                let nnn = op & 0x0FFF;

                self.pc = nnn;
            },
            (0x2,   _,   _,   _, ) => {
                let nnn = op & 0x0FFF;

                self.push(self.pc);
                self.pc = nnn;
            },
            (0x3,   _,   _,   _, ) => {
                let x = hex2 as usize;
                let nn = (op & 0x0FF) as u8;

                if self.v_reg[x] == nn {
                    self.pc += 2;
                }
            },
            (0x4,   _,   _,   _, ) => {
                let x = hex2 as usize;
                let nn = (op & 0x0FF) as u8;

                if self.v_reg[x] != nn {
                    self.pc += 2;
                }
            },
            (0x5,   _,   _,   0, ) => {
                let x = hex2 as usize;
                let y = hex3 as usize;

                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            },
            (0x6,   _,   _,   _, ) => {
                let x = hex2 as usize;
                let nn = (op & 0x0FF) as u8;

                self.v_reg[x] = nn;
            },
            (0x7,   _,   _,   _, ) => {
                let x = hex2 as usize;
                let nn = (op & 0x0FF) as u8;

                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            },
            (0x8,   _,   _,   0, ) => {
                let x = hex2 as usize;
                let y = hex3 as usize;

                self.v_reg[x] = self.v_reg[y];
            },
            (0x8,   _,   _,   1, ) => {
                let x = hex2 as usize;
                let y = hex3 as usize;

                self.v_reg[x] |= self.v_reg[y];
            },
            (0x8,   _,   _,   2, ) => {
                let x = hex2 as usize;
                let y = hex3 as usize;

                self.v_reg[x] &= self.v_reg[y];
            },
            (0x8,   _,   _,   3, ) => {
                let x = hex2 as usize;
                let y = hex3 as usize;

                self.v_reg[x] ^= self.v_reg[y];
            },
            (0x8,   _,   _,   4, ) => {
                let x = hex2 as usize;
                let y = hex3 as usize;

                let (new_vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let new_vf = if carry { 1 } else { 0 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },
            (0x8,   _,   _,   5, ) => {
                let x = hex2 as usize;
                let y = hex3 as usize;

                let (new_vx, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                let new_vf = if borrow { 0 } else { 1 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },
            (0x8,   _,   _,   6, ) => {
                let x = hex2 as usize;
           
                let lsb = self.v_reg[x] & 1;
                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;
            },
            (0x8,   _,   _,   7, ) => {
                let x = hex2 as usize;
                let y = hex3 as usize;

                let (new_vx, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                let new_vf = if borrow { 0 } else { 1 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },
            (0x8,   _,   _, 0xE, ) => {
                let x = hex2 as usize;
           
                let msb = (self.v_reg[x] >> 7) & 1;
                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
            },
            (0x9,   _,   _, 0x0, ) => {
                let x = hex2 as usize;
                let y = hex3 as usize;
           
                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
            },
            (0xA,   _,   _,   _, ) => {
                let nnn = op & 0x0FFF;
           
                self.i_reg = nnn;
            },
            (0xB,   _,   _,   _, ) => {
                let nnn = op & 0x0FFF;
           
                self.pc = (self.v_reg[0] as u16) + nnn;
            },
            (0xC,   _,   _,   _, ) => {
                let x = hex2 as usize;
                let nn = (op & 0x00FF) as u8;
                let rng: u8 = random();
                self.v_reg[x] = rng & nn;
            },
            (0xD,   _,   _,   _, ) => {
                let x_coord = self.v_reg[hex2 as usize] as  u16;
                let y_coord = self.v_reg[hex3 as usize] as  u16;
                let num_rows = hex4;

                let mut flipped = false;
                for y_line in 0..num_rows {
                    let addr = self.i_reg + y_line as u16;
                    let pixels = self.ram[addr as usize];

                    for x_line in 0..8 {
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            let idx = x + SCREEN_WIDTH * y;
                            flipped |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }

                if flipped {
                    self.v_reg[0xF] = 1;
                } else {
                    self.v_reg[0xF] = 0;
                }
            },
            (0xE,   _, 0x9, 0xE, ) => {
                let x = hex2 as usize;
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];
                if key {
                    self.pc += 2;
                }
            },
            (0xE,  _, 0xA, 0x1) => {
                let x = hex2 as usize;
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];
                if !key {
                    self.pc += 2;
                }
            },
            (0xF,  _, 0x0, 0x7) => {
                let x = hex2 as usize;
                self.v_reg[x] = self.dt;
            },
            (0xF,   _, 0x0, 0xA) => {
                let x = hex2 as usize;
                let mut pressed = false;
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_reg[x] = i as u8;
                        pressed = true;
                        break;
                    }
                    if !pressed {
                        self.pc -= 2;
                    }
                }
            },
            (0xF,  _, 0x1, 0x5) => {
                let x = hex2 as usize;
                self.dt = self.v_reg[x];
            },
            (0xF,  _, 0x1, 0x8) => {
                let x = hex2 as usize;
                self.st = self.v_reg[x];
            },
            (0xF,  _, 0x1, 0xE) => {
                let x = hex2 as usize;
                let vx = self.v_reg[x] as u16;
                self.i_reg = self.i_reg.wrapping_add(vx);
            },
            (0xF,  _, 0x2, 0x9) => {
                let x = hex2 as usize;
                let c = self.v_reg[x] as u16;
                self.i_reg = c * 5;
            },
            (0xF,  _, 0x3, 0x3) => {
                let x = hex2 as usize;
                let vx = self.v_reg[x] as f32;
                
                let hundreds = (vx / 100.0).floor() as u8;
                let     tens = (vx /  10.0).floor() as u8;
                let     ones = (vx %  10.0).floor() as u8;

                self.ram[(self.i_reg + 0) as usize] = hundreds;
                self.ram[(self.i_reg + 1) as usize] =     tens;
                self.ram[(self.i_reg + 2) as usize] =     ones;
            },
            (0xF,  _, 0x5, 0x5) => {
                let x = hex2 as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.ram[i + idx] = self.v_reg[idx];
                }
            },
            (0xF,  _, 0x6, 0x5) => {
                let x = hex2 as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.v_reg[idx] = self.ram[i + idx];
                }
            },
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

    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
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

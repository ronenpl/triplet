


const WINVECTORS: [i32; 8] = [0x1c0, 0x38, 0x7, 0x124, 0x92, 0x49, 0x111, 0x54];
const POWERS_OF_TWO: [i32; 9] = [1, 2, 4, 8, 16, 32, 64, 128, 256];

#[derive(Debug)]
pub struct Board {
    obb: i32,
    xbb: i32,
    playertomove: bool,
    buffer: i32,
}

impl Board {
    pub fn new() -> Self {
        Board {
            obb: 0,
            xbb: 0,
            playertomove: true,
            buffer: 0
        }
    }    
    pub fn mv(&mut self, pos: i32) -> Result<(), String> {
        if pos > 8 || pos < 0 {
            return Err(String::from("Either you put in a number bigger than 8, or you put in a negative number!"))
        }
        let offset = 256 >> pos;
        if self.playertomove {
            self.buffer = self.xbb;
            self.xbb += offset;
            self.playertomove = !self.playertomove;

        } else {
            self.buffer = self.obb;
            self.obb += offset;
            self.playertomove = !self.playertomove;
        }
        Ok(())
    }

    pub fn unmv(&mut self) {
        if self.playertomove {
            self.obb = self.buffer;
            self.playertomove = !self.playertomove;            
        } else {
            self.xbb = self.buffer;
            self.playertomove = !self.playertomove
        }
    }

    pub fn generate_board(&self) -> Result<[i32; 9], &str> {
        let mut out: [i32; 9];
        if self.xbb & self.obb != 0 {
            return Err("Invalid Entry!")
        }
        for i in 0..=8 {
            let cursor = 256 >> i;
            if cursor & self.xbb != 0 {
                out[i] = 1
            } else if cursor & self.obb != 0 {
                out[i] = 0
            } else {
                out[i] = 2
            }
        }
        Ok(out)

    }

    pub fn evaluate_pos(&self) -> i32 {
        use crate::WINVECTORS;
        let bb = self.xbb | self.obb;
        for vector in WINVECTORS {
            if vector & self.xbb == vector { return 1 }
            else if vector & self.obb == vector { return -1 }
            
        }
        if bb == 511 { return 0 } 
        2
    }

}

pub mod board_utils;
pub mod ai;


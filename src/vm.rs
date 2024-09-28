use std::io::Read;

use crate::runtime_error;
use crate::ops::{crazy, encrypt, incr, rotate};

pub struct MalbolgeVM {
    mem: Vec<u16>,
}

impl MalbolgeVM {
    const MEM_SIZE: usize = 59049;

    pub fn load(source: &[u8]) -> Self {
        let mut mem = Vec::with_capacity(MalbolgeVM::MEM_SIZE);

        // Validate and load the program itself
        for (i, byte) in source.iter().copied().filter(|ch| !ch.is_ascii_whitespace()).enumerate() {
            if i >= MalbolgeVM::MEM_SIZE {
                runtime_error!("Program too long.");
            }

            if !matches!(byte, b'!'..=b'~') || !matches!((i + byte as usize) % 94, 4 | 5 | 23 | 39 | 40 | 62 | 68 | 81) {
                runtime_error!("Malformed program.");
            }

            mem.push(byte as u16);
        }

        if mem.is_empty() {
            runtime_error!("Empty programs not allowed.");
        } else if mem.len() == 1 {
            mem.push(crazy(mem[0], 0));
        }

        for i in mem.len()..MalbolgeVM::MEM_SIZE {
            mem.push(crazy(mem[i - 1], mem[i - 2]));
        }

        Self { mem }
    }

    pub fn run(mut self) {
        let mut stdin = std::io::stdin().bytes();
        let (mut acc, mut cp, mut dp) = (0, 0, 0);

        loop {
            let op = self.read(cp);
            ensure_valid(op);

            match (op + cp) % 94 {
                4 => {
                    cp = self.read(dp);
                    ensure_valid(self.read(cp)); // Ensure the instruction we jumped to can be encrypted
                },
                5 => print!("{}", (acc as u8 as char)),
                23 => acc = stdin.next().map(|x| x.unwrap() as u16).unwrap_or(59048),
                39 => acc = self.write(dp, rotate(self.read(dp))),
                40 => dp = self.read(dp),
                62 => acc = self.write(dp, crazy(acc, self.read(dp))),
                81 => return,
                 _ => {}, // NOP
            }

            self.write(cp, encrypt(self.read(cp)));
            cp = incr(cp);
            dp = incr(dp);
        }
    }

    // Utilities to avoid having to sprinkle usize casts everywhere
    fn read(&self, pos: u16) -> u16 {
        self.mem[pos as usize]
    }

    fn write(&mut self, pos: u16, val: u16) -> u16 {
        self.mem[pos as usize] = val;
        val // Returns the written value
    }
}

fn ensure_valid(x: u16) {
    if !matches!(x, 33..=126) {
        runtime_error!("Runtime error.");
    }
}

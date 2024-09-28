use std::io::Read;

use crate::runtime_error;
use crate::value::Value;

static ENCRYPT_TABLE: [u16; 94] = [
    53, 122, 93, 38, 103, 113, 116, 121, 102, 114,
    36, 40, 119, 101, 52, 123, 87, 80, 41, 72, 45,
    90, 110, 44, 91, 37, 92, 51, 100, 76, 43, 81,
    59, 62, 85, 33, 112, 74, 83, 55, 50, 70, 104,
    79, 65, 49, 67, 66, 54, 118, 94, 61, 73, 95,
    48, 47, 56, 124, 106, 115, 98, 57, 109, 60,
    46, 84, 86, 97, 99, 96, 117, 89, 42, 77, 75,
    39, 88, 126, 120, 68, 108, 125, 82, 69, 111,
    107, 78, 58, 35, 63, 71, 34, 105, 64
];

pub struct MalbolgeVM {
    acc: Value,
    cp: Value,
    dp: Value,
    mem: Vec<Value>,
}

impl MalbolgeVM {
    const MEM_SIZE: usize = 59049;

    pub fn init(source: &str) -> Self {
        let mem = load_program(source);
        Self { mem, acc: Value::zero(), cp: Value::zero(), dp: Value::zero() }
    }

    pub fn run(mut self) {
        let mut stdin = std::io::stdin().bytes();

        loop {
            let op = self.mem[self.cp].val();
            if op < 33 || op > 126 {
                runtime_error!("Runtime error.");
            }

            match (op + self.cp.val()) % 94 {
                4 => self.cp = self.mem[self.dp],
                5 => print!("{}", (self.acc.val() as u8) as char),
                23 => {
                    let val = if let Some(byte) = stdin.next() {
                        byte.unwrap() as u16
                    } else {
                        Value::MAX
                    };
                    self.acc = Value::new(val);
                },
                39 => {
                    self.acc = self.mem[self.dp].shr();
                    self.mem[self.dp] = self.acc;
                },
                40 => self.dp = self.mem[self.dp],
                62 => {
                    self.acc = self.acc.crz(self.mem[self.dp]);
                    self.mem[self.dp] = self.acc;
                },
                81 => return,
                 _ => {}, // NOP
            }

            // Encrypt the previous memory position and increment pointers
            self.mem[self.cp] = Value::new(ENCRYPT_TABLE[self.mem[self.cp].val() as usize - 33]);
            self.cp.incr();
            self.dp.incr();
        }
    }
}

fn load_program(source: &str) -> Vec<Value> {
    let mut mem = Vec::with_capacity(MalbolgeVM::MEM_SIZE);

    // Validate and load the program itself
    for (i, ch) in source.chars().filter(|ch| !ch.is_ascii_whitespace()).enumerate() {
        if i >= MalbolgeVM::MEM_SIZE {
            runtime_error!("Program too long.");
        }

        if !matches!(ch, '!'..='~') || !matches!((i + ch as usize) % 94, 4 | 5 | 23 | 39 | 40 | 62 | 68 | 81) {
            runtime_error!("Malformed program.");
        }

        mem.push(Value::new(ch as u16));
    }

    // Fill up the rest of the memory using the crazy operation on the two previous addresses.
    // Note: both the official spec and the reference interpreter don't consider the case
    // of 1-character programs, where the second previous address to the first memory
    // position to fill up is OOB. In that case, we'll just consider it to be zeroed out.
    if mem.len() == 1 {
        mem.push(mem[0].crz(Value::zero()));
    }

    for i in mem.len()..MalbolgeVM::MEM_SIZE {
        mem.push(mem[i-1].crz(mem[i-2]));
    }

    mem
}

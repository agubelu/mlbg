mod vm;
mod value;

use std::fs::read_to_string;
use std::env::args;

#[macro_export]
macro_rules! runtime_error {
    // Using a macro for this because the ! makes it look more intimidating
    ($msg: expr) => {
        eprintln!($msg);
        std::process::exit(1);
    };
}

fn main() {
    let file_path = args().nth(1);
    if file_path.is_none() {
        runtime_error!("Usage: mlbg program.mb");
    }

    let source = read_to_string(file_path.unwrap()).expect("Could not read file.");
    let vm = vm::MalbolgeVM::init(&source);
    vm.run();
}

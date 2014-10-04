extern crate um;

use um::cpu::Instruction;
use std::io::File;

fn main() {
    let mut f = File::open(&Path::new(std::os::args()[1].clone()));
    let mut prog = Vec::<u32>::new();

    loop {
        match f.read_be_u32() {
            Ok(n) => {
                prog.push(n);
            }
            Err(_) => break,
        };
    }

    for (k, v) in prog.iter().map(|x| Instruction::from_u32(*x)).enumerate() {
        println!("{}: {}", k, v);
    }
}

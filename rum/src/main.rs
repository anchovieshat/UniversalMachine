extern crate um;

use um::cpu::Instruction;
use std::io::File;

fn main() {
    let mut state = um::cpu::State::new();
    let mut f = File::open(&Path::new(std::os::args()[1].clone()));
    let mut prog = Vec::<u32>::new();

    loop {
        match f.read_be_u32() {
            Ok(n) => {
                //println!("Read {:t} ({:X})", n, n);
                prog.push(n)
            }
            Err(_) => break
        }
    }

    if prog.len() == 0 {
        fail!("Empty program!");
    }

    state.load(&prog);
    state.run();
}

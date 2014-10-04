extern crate um;

use um::cpu::Instruction;
use std::io::File;
use std::task;
use std::sync::{Arc, Mutex};

fn main() {
    let mut f = File::open(&Path::new(std::os::args()[1].clone()));
    let mut f = f.unwrap();
    let mut prog = Vec::<u32>::with_capacity(f.stat().unwrap().size as uint / 4);
    let mut state = um::cpu::State::new();

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

    let state = Arc::new(Mutex::new(state));

    let istate = state.clone();
    let result = task::try(proc(){
        let mut istate = istate.lock();
        istate.load(&prog);
        istate.run();
    });


    let state = state.lock();
    match result {
        Ok(_) => println!("Blah"),
        Err(_) => {
            state.dump_arrays();
            state.dump_regs();
        },
    };
}

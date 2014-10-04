use std::io::{File, BufferedWriter};
use std::io::stdio::{stdin_raw, stderr_raw};
use std::collections::HashMap;
use std::collections::hashmap::{Occupied, Vacant};


/*use phf::PhfMap;

static INST2STR: PhfMap<u32, &'static str> = phf_map! {
    0 => "CMOV",
    1 => "AGET",
    2 => "ASET",
    3 => "ADD",
    4 => "MUL",
    5 => "DIV",
    6 => "NAND",
    7 => "HALT",
    8 => "NEWARR",
    9 => "DELARR",
    10 => "OUT",
    11 => "IN",
    12 => "LOAD",
    13 => "MOVE"
};*/


#[deriving(Show)]
pub struct Register(u32);

impl Register {
    pub fn as_u32(self) -> u32 {
        let Register(a) = self;
        a
    }
    pub fn set(&mut self, val: u32) {
        *self = Register(val);
    }
}

pub struct State {
    registers: [Register, ..8],
    arrays: HashMap<u32, Vec<u32>>,
    next_array: u32,
    pc: u32,
    running: bool,
    outfile: Option<BufferedWriter<File>>
}

impl State {
    pub fn new() -> State {
        State {
            registers: [Register(0), ..8],
            arrays: HashMap::<u32, Vec<u32>>::new(),
            next_array: 1,
            pc: 0,
            running: true,
            outfile: None,
        }
    }
    fn rget(&self, reg: u32) -> u32 {
        self.registers[reg as uint].as_u32()
    }
    fn rset(&mut self, reg: u32, val: u32) {
        self.registers[reg as uint].set(val);
    }
    pub fn load(&mut self, prog: &Vec<u32>) {
        match self.arrays.entry(0) {
            Occupied(mut ent) => {
                ent.set(prog.clone());
            }
            Vacant(mut ent) => {
                ent.set(prog.clone());
            }
        };
        self.pc = 0;
    }
    pub fn run(&mut self) {
        while self.running && self.pc < (self.arrays.get(&0).len() as u32) {
            let inst = Instruction::from_u32(*self.arrays.get(&0).get(self.pc as uint));
            self.pc = self.pc+1;
            self.exec(&inst);
        }
    }
    fn exec(&mut self, inst: &Instruction) {
        //println!("{}", *inst);
        match *inst {
            Cmov(dest, src, cond) => {
                let newval = self.rget(src);
                if self.rget(cond) != 0 {
                    self.rset(dest, newval)
                }
            }
            Aget(dest, arrr, idxr) => {
                //self.dump_arrays();
                //self.dump_regs();
                let idx = self.rget(idxr);
                let arr = self.rget(arrr);
                let newval = *self.arrays.get_mut(&arr).get_mut(idx as uint);
                self.rset(dest, newval);
            }
            Aset(arrr, idxr, valr) => {
                let idx = self.rget(idxr);
                let arr = self.rget(arrr);
                let val = self.rget(valr);
                let nval = self.arrays.get_mut(&arr).get_mut(idx as uint);
                *nval = val;
            }
            Add(dest, var1, var2) => {
                let newval = self.rget(var1) + self.rget(var2);
                self.rset(dest, newval);
            }
            Mul(dest, var1, var2) => {
                let newval = self.rget(var1) * self.rget(var2);
                self.rset(dest, newval);
            }
            Div(dest, var1, var2) => {
                let newval = self.rget(var1) / self.rget(var2);
                self.rset(dest, newval);
            }
            Nand(dest, var1, var2) => {
                let newval = !((self.rget(var1)) & (self.rget(var2)));
                self.rset(dest, newval);
            }
            Halt => {
                self.running = false
            }
            NewArr(dest, sizer) => {
                let size = self.rget(sizer);
                let next = self.next_array;
                self.arrays.insert(next, Vec::<u32>::from_fn(size as uint, |_|0));
                self.rset(dest, next);
                self.next_array = next + 1;
            }
            DelArr(idxr) => {
                let idx = self.rget(idxr);
                self.arrays.remove(&idx);
            }
            Out(reg) => {
                let ch = self.rget(reg);
                match self.outfile {
                    Some(ref mut file) => {
                        file.write_u8(ch as u8);
                    }
                    None => {
                        stderr_raw().write_u8(ch as u8);
                    }
                }
            }
            In(reg) => {
                let ch = stdin_raw().read_byte().unwrap();
                /*if ch == ('p' as u8) {
                    self.outfile = Some(BufferedWriter::new(File::create(&Path::new("out")).unwrap()));
                    println!("Outputting to file now...");
                }*/
                self.rset(reg, ch as u32);
            }
            Load(array, newpc) => {
                let arr = self.rget(array);
                if arr != 0 {
                    *(self.arrays.get_mut(&0)) = self.arrays.get(&arr).clone();
                }
                self.pc = self.rget(newpc);
            }
            Move(dest, value) => {
                self.rset(dest, value);
            }
            Unknown(ins) => fail!("Attempt to execute unknown instruction: {}", ins)
        }
    }
    pub fn dump_regs(&self) {
        for (i, x) in self.registers.iter().enumerate() {
            write!(stderr_raw(), "{}: {}\n", i, x);
        }
    }
    pub fn dump_arrays(&self) {
        for (k, v) in self.arrays.iter() {
            write!(stderr_raw(), "{}: {}\n", k, v);
        }
    }
}

#[deriving(Show)]
#[deriving(PartialEq)]
#[deriving(Eq)]
pub enum Instruction {
    Cmov(u32, u32, u32), // Source, Destination, Condition
    Aget(u32, u32, u32), // Destination, Array, Index
    Aset(u32, u32, u32), // Array, Index, Value
    Add(u32, u32, u32), // Source1, Source2, Destination
    Mul(u32, u32, u32), // Source1, Source2, Destination
    Div(u32, u32, u32),  // Source1, Source2, Destination
    Nand(u32, u32, u32), // Source1, Source2, Destination
    Halt,
    NewArr(u32, u32), //Destination, Index
    DelArr(u32), //u32 to Delete
    Out(u32), //u32 to take in
    In(u32), //u32 to print
    Load(u32, u32), //Source1, Source2
    Move(u32, u32), //Destination, Value
    Unknown(u32)
}

impl Instruction {
    pub fn from_u32(oper: u32) -> Instruction {
        let a = (oper & 0b00000000000000000000000111000000) >> 6;
        let b = (oper & 0b00000000000000000000000000111000) >> 3;
        let c = (oper & 0b00000000000000000000000000000111)     ;
        match (oper &   0b11110000000000000000000000000000) >> 28 {
            0 => Cmov(a, b, c),
            1 => Aget(a, b, c),
            2 => Aset(a, b, c),
            3 => Add(a, b, c),
            4 => Mul(a, b, c),
            5 => Div(a, b, c),
            6 => Nand(a, b, c),
            7 => Halt,
            8 => NewArr(b, c),
            9 => DelArr(c),
            10 => Out(c),
            11 => In(c),
            12 => Load(b, c),
            13 => Move((oper & 0b00001110000000000000000000000000) >> 25 , oper & 0b00000001111111111111111111111111),
            a => Unknown(a),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Instruction, Cmov, Move};
    #[inline(always)]
    fn inst_match(num: u32, inst: &Instruction) {
        println!("{} == {}", Instruction::from_u32(num), *inst);
        assert!(Instruction::from_u32(num) == *inst);
    }
    #[test]
    fn test_parse_inst() {
        inst_match(0b00000000000000000000000011010001, &Cmov(0b011, 0b010, 0b001));
        inst_match(0b11010110000000000000000000001000, &Move(0b011, 0b1000));
    }
}


use std::{
    io::{
        self,
        prelude::*,
        stdin,
        BufReader,
    },
};

enum Instr {
    Nop,
    Acc(isize),
    Jmp(isize),
}

impl From<&str> for Instr {
    fn from(s : &str) -> Instr {
        let mut cols = s.split(" ");
        let instr = cols.next().unwrap();
        let arg =  cols.next().and_then(|s| s.parse::<isize>().ok()).unwrap();

        match instr {
            "nop" => Instr::Nop,
            "jmp" => Instr::Jmp(arg),
            "acc" => Instr::Acc(arg),
            _ => unreachable!(),
        }

    }
}


struct Vm {
    acc : isize,
    pc : isize,
    text : Vec<Instr>,
}

impl Vm {
    fn new<F: io::BufRead>(f : F) -> Result<Vm, io::Error> {
        let mut ret = Vm {
            acc : 0,
            pc : 0,
            text : Vec::new(),
        };

        for line in f.lines() {
            ret.text.push(line?.as_str().into());
        }

        Ok(ret)
    }

    fn step(&mut self) {
        match self.text[self.pc as usize] {
            Instr::Nop => self.pc += 1,
            Instr::Acc(off) => {
                self.acc += off;
                self.pc += 1;
            },
            Instr::Jmp(off) => self.pc += off,
        }

    }
}

fn main() {
    let mut vm = Vm::new(BufReader::new(stdin())).unwrap();
    let mut visited = Vec::new();
    visited.resize(vm.text.len(), false);

    loop {
        if visited[vm.pc as usize] {
            break;
        }
        visited[vm.pc as usize] = true;
        vm.step();
    }

    println!("{}", vm.acc);
}

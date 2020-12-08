
use std::{
    io::{
        self,
        stdin,
        BufReader,
    },
    convert::{
        TryFrom,
        TryInto,
    },
};

enum Instr {
    Nop,
    Acc(isize),
    Jmp(isize),
}

impl TryFrom<&str> for Instr {
    type Error = String;

    fn try_from(s : &str) -> Result<Instr, String>{
        let mut cols = s.split(" ");
        let instr = cols.next().ok_or("no instr".to_string())?;
        let arg =  cols.next()
                       .ok_or("no arg".to_string())
                       .and_then(|s|
                            s.parse().or(Err("invalid string".to_string())))?;

        Ok(match instr {
            "nop" => Instr::Nop,
            "jmp" => Instr::Jmp(arg),
            "acc" => Instr::Acc(arg),
            _ => return Err("invalid instr".to_string())
        })

    }
}



struct Vm {
    acc : isize,
    pc : isize,
    text : Vec<Instr>,
}

impl Vm {
    fn new<F: io::BufRead>(f : F) -> Result<Vm, String> {
        let mut ret = Vm {
            acc : 0,
            pc : 0,
            text : Vec::new(),
        };

        for line in f.lines() {
            ret.text.push(line
                            .map_err(|e| format!("{}", e))?
                            .as_str()
                            .try_into()?);
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

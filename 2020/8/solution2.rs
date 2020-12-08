
use std::{
    io::{
        self,
        prelude::*,
        stdin,
        BufReader,
    },
};

#[derive(Clone, Copy)]
enum Instr {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl From<&str> for Instr {
    fn from(s : &str) -> Instr {
        let mut cols = s.split(" ");
        let instr = cols.next().unwrap();
        let arg =  cols.next().and_then(|s| s.parse::<isize>().ok()).unwrap();

        match instr {
            "nop" => Instr::Nop(arg),
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

    fn save_state(&self) -> (isize, isize) {
        return (self.acc, self.pc)
    }

    fn load_state(&mut self, state : (isize, isize)) {
        self.acc = state.0;
        self.pc = state.1;
    }

    fn get_current_instr(&self) -> Instr {
        self.text[self.pc as usize]
    }

    fn set_current_instr(&mut self, instr: Instr) {
        self.text[self.pc as usize] = instr;
    }

    fn reset(&mut self) {
        self.acc = 0;
        self.pc = 0;
    }

    fn step(&mut self) {
        match self.text[self.pc as usize] {
            Instr::Nop(_) => self.pc += 1,
            Instr::Acc(off) => {
                self.acc += off;
                self.pc += 1;
            },
            Instr::Jmp(off) => self.pc += off,
        }

    }
}

fn does_halt(vm : &mut Vm, visited : &mut Vec<bool>) -> bool {
    let mut visited = Vec::new();
    visited.resize(vm.text.len(), false);

    loop {
        if visited[vm.pc as usize] {
            return false
        }
        visited[vm.pc as usize] = true;
        vm.step();

        if vm.pc as usize == visited.len() {
            return true
        }
    }

}

fn find_halt(vm : &mut Vm, visited : &mut Vec<bool>, has_swaped : bool) -> bool {

    if vm.pc as usize == vm.text.len() {
        return true
    }

    if visited[vm.pc as usize] {
        return false
    } else {
        visited[vm.pc as usize] = true;
    }

    let state = vm.save_state();
    vm.step();

    if find_halt(vm, visited, has_swaped) {
        true
    } else if has_swaped {
        visited[vm.pc as usize] = false;
        false
    } else {

        vm.load_state(state);

        let old = vm.get_current_instr();

        vm.set_current_instr(match old {
            Instr::Acc(_) => {
                visited[vm.pc as usize] = false;
                return false;
            },
            Instr::Nop(n) => Instr::Jmp(n),
            Instr::Jmp(n) => Instr::Nop(n),
        });

        vm.step();

        if find_halt(vm, visited, true) {
            true
        } else {
            vm.load_state(state);
            vm.set_current_instr(old);
            visited[vm.pc as usize] = false;

            false
        }
    }
}

fn main() {
    let mut vm = Vm::new(BufReader::new(stdin())).unwrap();
    let mut visited = Vec::new();
    visited.resize(vm.text.len(), false);

    find_halt(&mut vm, &mut visited, false);

    println!("{}", vm.acc);
}

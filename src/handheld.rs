use crate::handheld::Instr::{Nop, Acc, Jmp};
use std::collections::HashSet;
use std::cell::Cell;

#[derive(Copy, Clone)]
pub enum Instr {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

pub struct Program {
    instrs :  Vec<Instr>,
    acc: Cell<i32>,
    loc: Cell<i32>
}

impl Program {

    pub fn new(instrs : Vec<Instr>) -> Program {
        Program{
            instrs,
            acc: Cell::new(0),
            loc: Cell::new(0),
        }
    }

    pub fn eval_loop(&mut self) -> i32 {
        let mut seen = HashSet::new();
        while ! seen.contains(&self.loc.get()) {
            seen.insert(self.loc.get());
            self.step(false);
        }
        self.acc.get()
    }

    pub fn fix_eval(&mut self) -> i32 {
        let mut op_locs = self.op_locs();
        for loc in op_locs {
            let m_result = self.fix_op_at(loc);
            if m_result.is_some() {
                return m_result.unwrap();
            }
            self.reset();
        }
        self.acc.get()
    }

    fn fix_op_at(&mut self, loc : usize) -> Option<i32> {
        let mut seen = HashSet::new();
        while ! seen.contains(&self.loc.get()) {
            seen.insert(self.loc.get());
            let swap= loc == self.loc.get() as usize;
            self.step(swap);
            if self.loc.get() >= self.instrs.len() as i32 {
                return Some(self.acc.get());
            }
        }
        None
    }

    fn op_locs(&self) -> Vec<usize> {
        let mut locs = Vec::new();
        for (i, op) in self.instrs.iter().enumerate() {
            match op {
                Nop(_) => (),
                op=> locs.push(i)
            }
        }
        locs
    }

    fn step(&mut self, swap : bool) {
        let mut loc = self.loc.get();
        let mut acc = self.acc.get();
        let curr_op = self.instrs[(loc as usize)];

        match curr_op {
            Nop(i) => {
                if swap {
                    loc += i;
                } else {
                    loc += 1;
                }
            },
            Acc(i) => {
                acc += i;
                loc += 1;
            },
            Jmp(i) => {
                if swap {
                    loc += 1
                } else {
                    loc += i;
                }
            }
        }
        self.loc.set(loc);
        self.acc.set(acc);
    }

    fn reset(&mut self) {
        self.loc.set(0);
        self.acc.set(0);
    }

}

pub fn parse(lines : Vec<&str>) -> Program {
    let mut instrs : Vec<Instr> = Vec::new();
    for line in lines {
        let split : Vec<&str> = line.split(' ').collect();
        let num : i32 = split[1].parse().unwrap();
        match split[0] {
            "nop" => instrs.push(Nop(num)),
            "acc" => instrs.push(Acc(num)),
            "jmp" => instrs.push(Jmp(num)),
            other => panic!("Unknown instruction found {}", other)
        }
    }
    Program::new(instrs)
}
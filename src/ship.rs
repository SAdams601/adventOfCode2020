use crate::ship::Dir::*;

#[derive(Debug,Copy,Clone,PartialEq)]
enum Dir {
    N = 0,
    E = 1,
    S = 2,
    W = 3
}

impl Dir {
    fn from_int(i: i32) -> Dir {
        match i {
            0 => N,
            1 => E,
            2 => S,
            3 => W,
            i => panic!("Trying to convert {} to direction", i)
        }
    }
}

struct Ship {
    x: i32,
    y: i32,
    facing: Dir
}

enum Instr {
    Forward(i32),
    Turn(i32,i32),
    Move(Dir, i32)
}

impl Ship {
    pub fn new() -> Ship {
        Ship {x: 0,y: 0,facing: E}
    }

    pub fn dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}
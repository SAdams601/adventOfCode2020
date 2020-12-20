use crate::ship::Dir::*;
use crate::ship::Instr::*;
use std::cell::Cell;

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Dir {
    N = 0,
    E = 1,
    S = 2,
    W = 3
}

fn from_char(c : char) -> Dir {
    match c {
        'N' => N,
        'E' => E,
        'S' => S,
        'W' => W,
        c => panic!("Failed to parse {} to a dir", c)        
    }
}
fn from_int(i: i32) -> Dir {
    match i {
        0 => N,
        1 => E,
        2 => S,
        3 => W,
        i => panic!("Trying to convert {} to direction", i)
    }
}

impl Dir {
    fn turning(&self, way : i32, degrees: i32) -> Dir {
        let steps = (degrees / 90) * way;
        let self_num = (*self as i32);
        let new_steps = (self_num + steps).rem_euclid(4);
        from_int(new_steps)
    } 
}

pub struct Ship {
    x: Cell<i32>,
    y: Cell<i32>,
    facing: Cell<Dir>
}

#[derive(Debug)]
pub enum Instr {
    Forward(i32),
    Turn(i32,i32),
    Move(Dir, i32)
}

impl Ship {
    pub fn new() -> Ship {
        Ship {x: Cell::new(0),y: Cell::new(0),facing: Cell::new(E)}
    }

    pub fn pos(&self) -> (i32, i32, Dir) {
        (self.x.get(), self.y.get(), self.facing.get())
    }

    pub fn dist(&self) -> i32 {
        self.x.get().abs() + self.y.get().abs()
    }

    pub fn run(&self, i : Instr) {
        println!("Running {:?}", i);
        match i {
            Forward(i) => self.sail_forward(i),
            Turn(way, degrees) => self.turn(way, degrees),
            Move(dir, amt) => self.sail(dir, amt)

        }
    }

    fn turn(&self, way : i32, degrees: i32) {
        let curr_facing = self.facing.get();
        let new_facing = curr_facing.turning(way, degrees);
        println!("Turning from {:?}, to {:?}", curr_facing, new_facing);
        self.facing.set(new_facing);
    }

    fn sail_forward(&self, dist: i32) {
        self.sail(self.facing.get(), dist);
    }

    fn sail(&self, dir: Dir, dist: i32) {
        let curr_x = self.x.get();
        let curr_y = self.y.get();
        match dir {
            N => self.y.set(curr_y + dist),
            E => self.x.set(curr_x + dist),
            S => self.y.set(curr_y - dist),
            W => self.x.set(curr_x - dist),
        }
    }
}

pub fn parse(line: &str) -> Instr {
    let instr_type = line.chars().take(1).collect::<Vec<char>>()[0];
    let num: i32 = line[1..].parse().expect("Could not parse number");
    println!("Parsing {:?} with num {:?}", instr_type, num);
    match instr_type {
        'F' => Forward(num),
        'L' => Turn(-1,num),
        'R' => Turn(1,num),
         c => Move(from_char(c), num)
    }
}
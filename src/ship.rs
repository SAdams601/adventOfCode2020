use crate::ship::Dir::*;
use crate::ship::Instr::*;
use std::cell::Cell;
use std::f64::consts::PI;

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

#[derive(Debug)]
pub struct Waypoint {
    x_offset : Cell<i32>,
    y_offset : Cell<i32>
}

impl Waypoint {

    fn new() -> Waypoint {
        Waypoint { x_offset: Cell::new(10), y_offset: Cell::new(1) }
    }

    fn mv(&self, dir: Dir, amount: i32) {
        let curr_y = self.y_offset.get();
        let curr_x = self.x_offset.get();
        match dir {
            N => self.y_offset.set(curr_y + amount),
            E => self.x_offset.set(curr_x + amount),
            S => self.y_offset.set(curr_y - amount),
            W => self.x_offset.set(curr_x - amount)
        }
    }

    fn rotate(&self, way: i32, degrees: i32) {
        let radians = -deg_to_rad(degrees) * way as f64;
        let curr_x = self.x_offset.get();
        let curr_y = self.y_offset.get();
        let r : f64 = ((curr_x.pow(2) + curr_y.pow(2)) as f64).sqrt();
        let mut phi = (curr_x as f64 /r).acos();
        if curr_y < 0 {
            phi *= -1.0;
        }

        phi += radians;
        let new_x = (r * phi.cos()).round();
        let new_y = (r * phi.sin()).round();
        self.x_offset.set(new_x as i32);
        self.y_offset.set((new_y) as i32);
    }
}

fn deg_to_rad(degrees : i32) -> f64 {
    (degrees as f64) * (PI/180.0)
}

fn rad_to_deg(radians : f64) -> i32 {
    (radians * (180.0/PI)) as i32
}

pub struct Ship {
    x: Cell<i32>,
    y: Cell<i32>,
    waypoint: Waypoint
}

#[derive(Debug)]
pub enum Instr {
    Forward(i32),
    Turn(i32,i32),
    Move(Dir, i32)
}

impl Ship {
    pub fn new() -> Ship {
        Ship {x: Cell::new(0),y: Cell::new(0),waypoint: Waypoint::new()}
    }

    pub fn pos(&self) -> (i32, i32, i32, i32) {
        (self.x.get(), self.y.get(), self.waypoint.x_offset.get(), self.waypoint.y_offset.get())
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
        self.waypoint.rotate(way, degrees);
    }

    fn sail_forward(&self, times: i32) {
        let curr_x = self.x.get();
        let curr_y = self.y.get();

        self.x.set(curr_x + (self.waypoint.x_offset.get() * times));
        self.y.set(curr_y + (self.waypoint.y_offset.get() * times));
    }

    fn sail(&self, dir: Dir, dist: i32) {
        self.waypoint.mv(dir, dist);
    }
}

pub fn parse(line: &str) -> Instr {
    let instr_type = line.chars().take(1).collect::<Vec<char>>()[0];
    let num: i32 = line[1..].parse().expect("Could not parse number");
    match instr_type {
        'F' => Forward(num),
        'L' => Turn(-1,num),
        'R' => Turn(1,num),
         c => Move(from_char(c), num)
    }
}
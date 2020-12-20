#![allow(warnings)]
#[macro_use]
extern crate lazy_static;

mod slope;
mod passport;
mod seats;
mod bags;
mod handheld;
mod xmas_cracker;
mod ferry;
mod ship;

use std::fs;
use crate::ship::*;

fn main() {
    let contents = fs::read_to_string("input/day12.txt")
        .expect("Error reading file").to_owned();
    let lines : Vec<&str> = contents.lines().collect();
    let ship = Ship::new();
    println!("Starting==========\n{:?}", lines);
    for line in lines {
        let instr = parse(line);
        ship.run(instr);
        println!("Ship pos {:?}", ship.pos());
    }

    println!("{}", ship.dist());
}
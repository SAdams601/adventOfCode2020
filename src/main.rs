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
mod bitmask;
mod memory;

use std::fs;
use crate::memory::play;

fn main() {
    let contents = fs::read_to_string("input/day15.txt")
        .expect("Error reading file").to_owned();
    let start: Vec<i32> = contents.split(',').map(|num| num.parse().expect("Found non number")).collect();
    let result = play(start);
    println!("{}", result);
}
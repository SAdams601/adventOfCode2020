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

use std::fs;
use crate::bitmask::run;

fn main() {
    let contents = fs::read_to_string("input/day14.txt")
        .expect("Error reading file").to_owned();
    let lines: Vec<&str> = contents.lines().collect();
    let result = run(lines);
    println!("{}", result);
}
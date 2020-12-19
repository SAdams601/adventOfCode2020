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

use std::fs;
use crate::ferry::Board;

fn main() {
    let contents = fs::read_to_string("input/day11.txt")
        .expect("Error reading file").to_owned();
    let lines : Vec<&str> = contents.lines().collect();
    let board = Board::parse(lines);
    let result = board.eval();
    println!("{:?}", result);
}
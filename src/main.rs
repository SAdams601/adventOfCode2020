#[macro_use]
extern crate lazy_static;

mod slope;
mod passport;
mod seats;
mod bags;
mod handheld;

use std::fs;
use std::borrow::Borrow;
use crate::handheld::parse;


fn main() -> Result<(), String>{
    let contents = fs::read_to_string("input/day8.txt")
        .expect("Error reading file").to_owned();
    let lines = contents.lines().collect();
    let mut program = parse(lines);
    let result = program.fix_eval();
    println!("{}", result);
    Ok(())
}



#[macro_use]
extern crate lazy_static;

mod slope;
mod passport;
mod seats;
mod bags;
mod handheld;
mod xmas_cracker;

use std::fs;
use std::borrow::Borrow;
use crate::handheld::parse;
use crate::xmas_cracker::Xmas;


fn main() -> Result<(), String>{
    let contents = fs::read_to_string("input/day9.txt")
        .expect("Error reading file").to_owned();
    let lines : Vec<&str> = contents.lines().collect();
    let nums : Vec<i64>= lines.iter().map(|ln| ln.parse().unwrap()).collect();
    let input = Xmas::new(nums, 25);
    let goal = input.find();
    let mut span = input.find_sum_range(goal);
    span.sort();
    println!("{:?}", span[0] + span[span.len() - 1]);
    Ok(())
}



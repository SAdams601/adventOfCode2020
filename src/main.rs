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
mod tickets;

use std::fs;
use crate::tickets::parse;
use crate::tickets::solve;

fn main() {
    let contents = fs::read_to_string("input/day16.txt")
        .expect("Error reading file").to_owned();
    let lines: Vec<String> = contents.lines().map(|ln| ln.to_string()).collect();
    let mut parsed = parse(&lines);
    let positions = solve(&parsed.fields, &parsed.other_tickets);
    let result = positions.iter().map(|&pos| &parsed.my_ticket[pos]).fold(1 as i64, |x,y| x * (*y as i64));
    println!("{}", result);
}
#[macro_use]
extern crate lazy_static;

mod slope;
mod passport;
mod seats;
mod bags;

use std::fs;
use std::borrow::Borrow;
use std::process::id;
use crate::bags::which_can_hold;

fn main() -> Result<(), String>{
    let read_results = fs::read_to_string("input/day7.txt")
        .expect("Error reading file").to_owned();
    let lines = read_results.lines().collect();
    let all_bags = crate::bags::parse(lines);
    let shiny_gold = all_bags.get("shiny gold").unwrap();
    let count = shiny_gold.count_contents(&all_bags);
    println!("{}", count);
    Ok(())
}



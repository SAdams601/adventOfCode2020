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


fn main() {
    let contents = fs::read_to_string("input/day13.txt")
        .expect("Error reading file").to_owned();
    let lines: Vec<&str> = contents.lines().collect();
    let currTime: i32 = lines[0].parse().expect("First line should be number");
    let busses: Vec<(i32,i32)>= lines[1].split(',').enumerate().filter_map(|(indx,id)| {
        if id == "x" {
            None
        } else {
            id.parse::<i32>().ok().and_then(|id_i32| Some((indx as i32, id_i32)))
        }
    }).collect();
    let t_min = 21.0;
    let result = solve(busses);
    println!("{:?}", result)
}

fn solve(busses : Vec<(i32, i32)>) -> i64 {
   let mut t = 0;
   let mut incr = busses[0].1 as i64;
   for j in 1..busses.len() {
        let (i, bus) = busses[j];
        loop {
            if (t + (i as i64)) % (bus as i64) == 0 {
                incr = (incr * (bus as i64));
                break;
            }
            t += incr;
    }
   }
   t
}
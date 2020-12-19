#![allow(warnings)]
#[macro_use]
extern crate lazy_static;

mod slope;
mod passport;
mod seats;
mod bags;
mod handheld;
mod xmas_cracker;

use std::fs;

fn main() -> Result<(), String>{
    let contents = fs::read_to_string("input/day10.txt")
        .expect("Error reading file").to_owned();
    let lines : Vec<&str> = contents.lines().collect();
    let mut nums : Vec<i32>= lines.iter().map(|ln| ln.parse().unwrap()).collect();
    nums.sort();
    let goal = nums.last().unwrap() + 3;
    nums.push(goal);    
    let mut j = 0;
    let mut diffs = Vec::new();
    for adapter in nums {
        diffs.push(adapter - j);
        j += adapter - j;
    }
    println!("{:?}", diffs);
    let result = count_branches(&diffs[..]);
    println!("{:?}", result);
    Ok(())
}

fn count_branches(diffs : &[i32]) -> i64 {
    let mut one_blocks = Vec::new();
    let mut current_count = 0;
    for diff in diffs {
        if *diff == 1 {
            current_count += 1;
        } else if current_count != 0 {
            one_blocks.push(current_count);
            current_count = 0;
        }
    }
    if current_count != 0 {

    }
    println!("{:?}",one_blocks);
    let mut result = 1;
    for block in one_blocks {
        match block {
            1 => (),
            2 => result *= 2,
            3 => result *= 4,
            4 => result *= 7,
            _ => ()
        }
    }
    result
}

fn count_next_possibilities(curr_jolts: i32, goal: i32, adapters: &[i32]) -> Vec<i32> {
    let mut j = curr_jolts;
    let mut diffs = Vec::new();
    for adapter in adapters {
        diffs.push(adapter - j);
        j += adapter - j;
    }
        
    diffs
}
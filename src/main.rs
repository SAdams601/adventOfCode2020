mod slope;
mod passport;
mod seats;

use std::fs;
use std::borrow::Borrow;
use std::process::id;
use std::collections::{HashSet, HashMap};

fn main() -> Result<(), String>{
    let lines : Vec<String> = fs::read_to_string("input/day6.txt")
        .expect("Error reading file").lines().map(str::to_string).collect();
    let groups = lines.iter().fold(Vec::new(),
                                   |mut acc, ln| {
        if ln.trim() == "" {
            acc.push(Vec::new())
        } else {
            let mut vec = acc.pop().map_or(Vec::new(), |v| v);
            vec.push(ln.trim().to_string());
            acc.push(vec);
        }
            acc
        });
    let mut sum = 0;
    for group in groups {
        let mut map = HashMap::new();
        let size : i32 = group.len() as i32;
        for answers in group {
            answers.chars().for_each(|c|
                {
                    let option = map.get(&c);
                    match option {
                        None => map.insert(c, 1),
                        Some(i) => map.insert(c, i+1)
                    };
                })
        }
        sum += map.values().filter(|i| i == &&size ).count();
    }
    println!("Total: {}", sum);
    Ok(())
}



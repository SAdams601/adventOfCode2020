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
use minilp::{Problem, OptimizationDirection, ComparisonOp};
use lp_modeler::solvers::{SolverTrait};
use lp_modeler::dsl::*;


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
    let result = solve(busses);
    println!("{:?}", result)
}

fn solve(busses : Vec<(i32, i32)>) -> i64 {
/*    let mut problem = Problem::new(OptimizationDirection::Minimize);
    let t_min = (busses.iter().map(|t| t.1).max().unwrap() + 1) as f64;
    let t = problem.add_var(1.0, (t_min, f64::INFINITY));

    for (i,bus) in busses {
        let c = problem.add_var(0.0, (t_min, f64::INFINITY));
        problem.add_constraint(&[(c, bus as f64), (t, -1.0)], ComparisonOp::Eq, i as f64);
    }
    let solution = problem.solve().unwrap();
    solution.objective() as i64*/

    let ref t = LpInteger::new("t");
    let mut problem = LpProblem::new("", LpObjective::Maximize);
    problem += t;

    for (i,bus) in busses {
        let c_name = format!("c_{}", i);
        let ref c = LpInteger::new(&c_name);
    }
    0
}
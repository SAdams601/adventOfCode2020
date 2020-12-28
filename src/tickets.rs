use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::cell::{RefCell, Cell};
use std::collections::hash_map::IntoIter;
use std::borrow::Borrow;

#[derive(Debug, Copy, Clone)]
struct Range {
    min : i32,
    max : i32
}

impl Range {
    fn within(&self, n: i32) -> bool {
        self.min <= n && n <= self.max
    }
}

#[derive(Debug)]
pub struct Field {
    name: String,
    ranges: Vec<Range>,
    positions: RefCell<HashSet<usize>>,
    invalid: RefCell<HashSet<usize>>,
    solved: Cell<bool>
}

impl Field {

    fn new(name: String, ranges: Vec<Range>) -> Field {
        Field {
            name,
            ranges,
            positions: RefCell::new(HashSet::new()),
            invalid: RefCell::new(HashSet::new()),
            solved: Cell::new(false)
        }
    }

    fn valid(&self, n : i32) -> bool {
        self.ranges.iter().any(|rng| rng.within(n))
    }

    fn add_if_valid(&mut self, (i, val) : (usize, i32)) {
        if self.valid(val) && ! self.invalid.borrow().contains(&i) {
            let set = self.positions.get_mut();
            set.insert(i);
        } else {
            self.positions.borrow_mut().remove(&i);
            self.invalid.borrow_mut().insert(i);
        }
    }

    fn check_if_solved(&mut self) -> Option<usize> {
        let mut positions = self.positions.borrow_mut();
        if positions.len() == 1 {
            self.solved.set(true);
            return Some(*positions.iter().collect::<Vec<&usize>>()[0]);
        }
        None
    }

    fn is_solved(&self) -> bool {
        self.solved.get()
    }
    fn remove_found(&self, found: &Vec<usize>) {
        let mut positions = self.positions.borrow_mut();
        for f in found {
            positions.remove(f);
        }
    }

    fn print_solution(&self) {
        println!("{}", self.name);
        println!("{:?}", self.positions);
    }

    fn get_position(&self) -> usize {
        *self.positions.borrow().iter().collect::<Vec<&usize>>()[0]
    }

}

pub struct Problem {
    pub(crate) fields: Vec<RefCell<Field>>,
    pub(crate) my_ticket: Vec<i32>,
    pub(crate) other_tickets: Vec<Vec<i32>>
}

pub fn parse(lines: &Vec<String>) -> Problem {
    let range_strs: Vec<&String> = lines.iter().take_while(|ln| "your ticket:" != *ln).to_owned().collect();
    let mut handled = range_strs.len();
    let my_ticket_str = &lines[handled..(handled+2)].to_owned();
    let other_tickets_strs = &lines[(handled+2)..].to_owned();

    let fields = parse_fields(range_strs);
    let my_ticket = parse_my_ticket(my_ticket_str);
    let other_tickets = parse_tickets(other_tickets_strs);
    Problem{
        fields,
        my_ticket,
        other_tickets
    }
}

fn is_valid_ticket(ticket : &Vec<i32>, fields: &Vec<RefCell<Field>>) -> bool {
    for val in ticket {
        if ! fields.iter().any(|f| f.borrow().valid(*val)) {
            return false;
        }
    }
    true
}

pub fn solve(fields: &Vec<RefCell<Field>>,  other_tickets: &Vec<Vec<i32>>) -> Vec<usize> {
    other_tickets.iter().filter(|tick| is_valid_ticket(tick, &fields))
        .for_each(|ticket| {
        for (i, &val) in ticket.iter().enumerate() {
            for field in fields.iter() {
                field.borrow_mut().add_if_valid((i,val));
            }
        }
    });
    let mut found_positions = Vec::new();
    while ! fields.iter().all(|rf| rf.borrow().is_solved()) {
        for field in fields.iter() {
            if ! field.borrow().is_solved() {
                field.borrow().remove_found(&found_positions);
                let m_solved = field.borrow_mut().check_if_solved();
                match m_solved {
                    None => (),
                    Some(i) => found_positions.push(i)
                }
            }
        }
    }
    fields.iter().take(6).map(|f| f.borrow().get_position()).collect()
}

fn not_solved(name: &str, solved: &Vec<(&str, usize)>) -> bool {
    solved.iter().any(|(s,_)| name == *s)
}

fn parse_tickets(strs: &Vec<String>) -> Vec<Vec<i32>> {
    let mut tickets = Vec::new();
    for str in strs {
        let matches = NUM_LIST_RE.find(str);
        if matches.is_some() {
            let nums = str.split(',');
            tickets.push(nums.map(|s| s.trim().parse().expect("Expected parsable number")).collect());
        }
    }
    tickets
}

lazy_static!{
    static ref NUM_LIST_RE : Regex = Regex::new(r"^(\d,?)+$").unwrap();
}

fn parse_my_ticket(strs: &Vec<String>) -> Vec<i32> {
    for str in strs {
        let matches = NUM_LIST_RE.find(str);
        if matches.is_some() {
            let nums = str.split(',');
            return nums.map(|s| s.trim().parse().expect("Expected parsable number")).collect();
        }
    }
    Vec::new()
}

fn parse_fields<'a>(strs: Vec<&String>) -> Vec<RefCell<Field>> {
    let mut fields = Vec::new();
    for str in strs {
        let split: Vec<&str> = str.split(':').collect();
        if split.len() > 1 {
            let name = split[0];
            let ranges = parse_ranges(split[1]);
            let mut field = Field::new(name.to_string(), ranges);
            fields.push(RefCell::new(field))
        }

    }
    fields
}

fn parse_ranges(str: &str) -> Vec<Range>{
    let mut ranges = Vec::new();
    let rng_strs = str.split("or");
    for rng in rng_strs {
        let nums: Vec<&str> = rng.split("-").collect();
        let min = nums[0].trim().parse().expect("Range should be a number");
        let max = nums[1].trim().parse().expect("Range should be a number");
        ranges.push(Range { min, max })
    }
    ranges
}
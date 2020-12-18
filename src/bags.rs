use std::collections::{HashMap, HashSet};
use regex::Regex;
use std::borrow::Borrow;
use std::hash::Hash;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::collections::hash_map::RandomState;

pub struct Bag<'a> {
    descriptor : &'a str,
    contains: Vec<BagsInside<'a>>
}

#[derive(Clone, Copy, Debug)]
struct BagsInside<'a> {
    descriptor : &'a str,
    num : i32
}

lazy_static! {
    static ref BAG_RE : Regex = Regex::new(r"^(?P<descriptor>\w+ \w+) bags contain (?P<bag_list>.+)\.$").unwrap();
    static ref BAG_LIST_RE: Regex = Regex::new(r"(?P<num>\d+) (?P<descriptor>\w+ \w+) bags?,?").unwrap();
}

impl Bag<'_> {
    fn bag_contains_one_of(&self, targets : &HashSet<&str>) -> bool {
        self.contains.iter().any(|bg|
            targets.get(bg.descriptor).is_some())

    }

    pub fn count_contents(&self, other_bags : &HashMap<String, Bag>) -> u32 {
        println!("Counting inside of {}", self.descriptor);
        let mut sum : u32 = 0;
        for inner_bag in &self.contains {
            let bag = other_bags.get(inner_bag.descriptor).unwrap();
            let num_inside = bag.count_contents(other_bags);
            let u_num = inner_bag.num as u32;
            sum += u_num + u_num * num_inside;
        }
        sum
    }
}


pub fn parse(lines: Vec<&str>) -> HashMap<String, Bag> {
    let mut map = HashMap::new();
    for line in lines {
        BAG_RE.captures(line.as_ref()).map(
            |m| {
                let outer_bag = m.name("descriptor").unwrap().as_str();
                let lst = parse_bag_list(m.name("bag_list").unwrap().as_str()).to_owned();
                map.insert(outer_bag.to_string(), Bag { descriptor: outer_bag, contains: lst });
            });
    }
    map
}

fn parse_bag_list(bag_list: &str) -> Vec<BagsInside> {
    let mut bag_results = Vec::new();
    if bag_list == "no other bags" {
        return bag_results;
    }
    let bag_strs = bag_list.split(",");
    for bag in bag_strs {
        BAG_LIST_RE.captures(bag.trim()).map(
            |m| {
                let descriptor = m.name("descriptor").unwrap().as_str();
                let num : i32 = m.name("num").unwrap().as_str().parse().unwrap();
                bag_results.push(BagsInside {descriptor, num});
            }
        );
    }
    bag_results
}

pub(crate) fn which_can_hold(target_bag : String, all_bags: Vec<Bag>) -> HashSet<&str> {
    let mut set : HashSet<&str> = HashSet::new();
    set.insert(&*target_bag);
    all_holders(&mut set, all_bags)
}

fn all_holders<'a>(targets: &mut HashSet<&str>, all_bags: Vec<Bag<'a>>) -> HashSet<&'a str> {
    let mut set= HashSet::new();
    let mut curr_targets = HashSet::new();
    let mut newly_added : HashSet<&str>;
    curr_targets.clone_from(targets);
    let mut added_this_round = -1;
    while added_this_round != 0 {
        println!("Current targets are: {:?}",curr_targets);
        added_this_round = 0;
        newly_added = HashSet::new();
        for bag in &all_bags {
            println!("searching {}", bag.descriptor);
            if ! set.contains(bag.descriptor) && bag.bag_contains_one_of(&curr_targets) {
                newly_added.insert(&*bag.descriptor);
                added_this_round += 1;
            }
        }
        println!("Adding {:?}", newly_added);
        newly_added.iter().for_each(|e| { set.insert(*e); } );
        curr_targets = newly_added;
    }
    set
}
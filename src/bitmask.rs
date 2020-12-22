use std::collections::HashMap;
use std::cell::Cell;
use std::cell::RefCell;
use regex::Regex;

struct State {
    mem:  HashMap<i64,Cell<i64>>,
    mask: Cell<[char;36]>
}

impl State {
    fn new() -> State {
        State {mem: HashMap::new(), mask: Cell::new(['0';36])}
    }

    fn set_mask(&self, mask: String) {
        self.mask.set(to_char_arr(mask));
    }

    fn set_mem(&mut self, addr: usize, num: i64) {
        let mut bits = to_bin_arr(addr);
        let addrs = self.apply_mask(bits);
        addrs.iter().for_each(|addr| {
            match self.mem.get(&addr) {
                Some(cell) => cell.set(num),
                _ => { self.mem.insert(*addr, Cell::new(num)); ()}
            }
        });
    }

    fn apply_mask(&self, mut bits: [char;36]) -> Vec<i64> {
        for (i,char) in self.mask.get().iter().enumerate() {
            match char {
                '1'|'X' => bits[i] = *char,
                '0'     => (),
                c => panic!("Could not apply mask {}", c)
            }
        }
        replace_floating(bits)
    }

    fn sum_mem(&self) -> i64 {
        let mut res = 0;
        for val in self.mem.values() {
            res += val.get();
        }
        res
    }
}

fn to_char_arr(str: String) -> [char;36] {
    let mut arr = ['0';36];
    for (i,c) in str.chars().enumerate() {
        arr[i] = c;
    }
    arr
}

fn to_bin_arr(num: usize) -> [char;36] {
    let mut arr = ['0';36];
    let mut vec: Vec<u32> = Vec::new();
    let mut rest = num;
    while rest > 0 {
        vec.push((rest % 2) as u32);
        rest /= 2;
    }
    vec.reverse();
    let start_i = 36 - vec.len();
    for i in 0..vec.len() {
        match vec[i] {
            0 => arr[start_i + i] = '0',
            1 => arr[start_i + i] = '1',
            i => panic!("Bit cannot be {}", i)
        }

    }
    arr
}

fn replace_floating(chars: [char;36]) -> Vec<i64> {
    let mut vec = vec![0];
    for (i, char) in chars.iter().enumerate() {
        let pow = 35 - i as u32;
        match char {
            '1' | '0' => {
                let curr_val = char::to_digit(*char, 10).unwrap() as i64 * 2_i64.pow(pow);
                vec = vec.iter().map(|i| i + curr_val).collect();
            },
            'X' => {
                let one_val = 2_i64.pow(pow);
                vec = vec.iter().flat_map(|&v| vec![v, v + one_val]).collect();
            },
            c=> panic!("Cannot add unknown character {}", c)
        }
    }
    vec
}

fn to_dec(bits: [char;36]) -> i64 {
    let mut pow= 36;
    let mut res = 0;
    for bit in bits.iter() {
        pow -= 1;
        let bit_num = char::to_digit(*bit, 10).unwrap() as i64;
        res += bit_num * 2_i64.pow(pow);
    }
    res
}

lazy_static! {
    static ref MEM_RE: Regex = Regex::new(r"^mem\[(?P<register>\d+)\] = (?P<value>\d+)$").unwrap();
}

pub fn run(lines: Vec<&str>) -> i64 {
    let mut state = State::new();
    for line in lines {
        if line.starts_with("mask") {
            let new_mask_str = line.split('=').last().unwrap().trim();
            state.set_mask(new_mask_str.to_string());
        } else {
            MEM_RE.captures(line).map(|capture| {
                let register: usize = capture.name("register").unwrap().as_str().parse().expect("Register should be number");
                let value: i64 = capture.name("value").unwrap().as_str().parse().expect("Value should be number");
                state.set_mem(register, value);
            });
        }
    }
    state.sum_mem()
}

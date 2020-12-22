use std::collections::HashMap;
use std::cell::Cell;
use Bitmask::Mask;

#[derive(Debug)]
enum Bitmask {
    Mask(Vec<(usize, u8)>)
}

impl Bitmask {

    fn new() -> Bitmask {
        Mask(Vec::new())
    }

    fn parse(msk: &str) -> Bitmask {
        let mut vec = Vec::new();
        for (i, c) in msk.chars().enumerate() {
            match c {
                '1' => vec.push((i, 1)),
                '0' => vec.push((i,0)),
                 _  => () 
            }
        }
        Mask(vec)
    }
}

struct State {
    mem:  HashMap<usize,Cell<[u8;36]>>,
    mask: Cell<Bitmask>
}

impl State {
    fn new() -> State {
        State {mem: HashMap::new(), mask: Cell::new(Bitmask::new())}
    }

    fn set_mask(&self, mask: Bitmask) {
        self.mask.set(mask);
    }

    fn set_mem(&mut self, addr: usize, num: i32) {
        let mut bits = self.apply_mask(to_bin_arr(num));
        match self.mem.get(&addr) {
            None => { self.mem.insert(addr, Cell::new(bits)); ()},
            Some(cell) => cell.set(bits)
        }
    }

    fn apply_mask(&self, bits: [u8;36]) -> [u8;36] {
        [0;36]
    }
}

fn to_bin_arr(num: i32) -> [u8;36] {
    let mut arr = [0;36];
    let mut vec: Vec<u8> = Vec::new();
    let mut rest = num;
    while rest >= 0 {
        vec.push((rest % 2) as u8);
        rest /= 2;
    }
    let start_i = 36 - vec.len();
    for i in 0..vec.len() {        
        arr[start_i + i] = vec[i];
    }
    arr
}

pub fn run(lines: Vec<&str>) -> i32 {
    for line in lines {
        if line.starts_with("mask") {

        } else {

        }
    }
    0
}

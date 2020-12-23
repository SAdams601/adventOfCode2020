use std::collections::HashMap;
use std::cell::Cell;
use std::borrow::Borrow;

struct GameState {
    seen: HashMap<i32, (Cell<usize>,Cell<usize> )>,
    turn: Cell<usize>,
    last_said: Cell<i32>
}

impl GameState {
    fn new(start: Vec<i32>) -> GameState {
        let turn = Cell::new(start.len() + 1);
        let mut seen = HashMap::new();
        for (i,x) in start.iter().enumerate() {
            seen.insert(*x, (Cell::new(0),Cell::new(i + 1)));
        }
        GameState{
            seen,
            turn,
            last_said: Cell::new(*start.last().unwrap())
        }
    }

    fn insert(&mut self, said : i32, turn: usize) {
        match self.seen.get_mut(&said) {
            None => {
                let v = (Cell::new(0), Cell::new(turn));
                self.seen.insert(said, v);
            },
            Some((c1, c2)) => {
                let old = c2.get();
                c1.set(old);
                c2.set(turn);
            }
        };
    }

    fn step(&mut self) -> usize {
        let last = self.last_said.get();
        let (c1,c2) = self.seen.get(&last).unwrap();
        let mut said = 0;
        if c1.get() == 0 {
            said = 0;
        } else {
            said = c2.get() - c1.get();
        }
        self.last_said.set(said as i32);
        let turn = self.turn.get();
        self.turn.set(turn + 1);
        self.insert(said as i32, turn);
        turn
    }

    fn play(&mut self) -> i32 {
        loop {
            let size = self.step();
            if size == 30000000 {
                break;
            } else {
                //println!("Size is {}", size);
            }
        }
        self.last_said.get()
    }
}

pub fn play(nums: Vec<i32>) -> i32 {
    let mut state = GameState::new(nums);
    state.play()
}
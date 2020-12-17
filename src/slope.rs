use crate::slope::Pos::{F, T};
use std::cell::Cell;

#[derive(Copy, Clone, PartialEq)]
pub enum Pos {
    T,
    F,
}

struct Board {
    h: usize,
    w: usize,
    board: Vec<Vec<Pos>>,
}

pub struct Slope {
    x: Cell<usize>,
    y: Cell<usize>,
    board: Board,
}

impl Slope {
    pub fn mv(&self, cols: usize, rows: usize) {
        self.x.set((self.x.get() + cols) % self.board.w);
        self.y.set(self.y.get() + rows);
    }

    pub fn curr_cell(&self) -> Pos {
        self.board.board[self.y.get()][self.x.get()].clone()
    }

    pub fn at_bottom(&self) -> bool {
        self.y.get() >= self.board.h
    }

    pub fn reset(&self) {
        self.x.set(0);
        self.y.set(0);
    }
}

pub(crate) fn parse_slope(board_str : Vec<String>) -> Slope {
    let cells : Vec<Vec<Pos>> = board_str.iter().map(string_to_cells).collect();
    let board = Board{h: cells.len(), w: cells[0].len(), board: cells};
    return Slope{x: Cell::new(0), y: Cell::new(0), board };
}

fn string_to_cells(s : &String ) -> Vec<Pos> {
    s.chars().map(|c| -> Pos { return if c == '#' { T } else { F }}).collect()
}


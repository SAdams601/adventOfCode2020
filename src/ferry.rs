use std::cell::RefCell;
use crate::ferry::Grid::*;
use core::fmt::Debug;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Grid {
    Floor,
    Empty,
    Occupied
}

#[derive(Debug)]
struct Pos {
    row : usize,
    col : usize,
    state : RefCell<Grid>
}

impl Pos {

    pub fn is_floor(&self) -> bool {
        let borrowed_state = self.state.borrow();
        *borrowed_state == Floor
    }    

    pub fn is_occupied(&self) -> bool {
        let borrowed_state = self.state.borrow();
        *borrowed_state == Occupied
    }

    pub fn is_empty(&self) -> bool {
        let borrowed_state = self.state.borrow();
        *borrowed_state == Empty
    }

    pub fn set_state(&self, new_state : Grid) {
        self.state.replace(new_state);
    }
}

#[derive(Debug)]
pub struct Board {
    rows : usize,
    cols : usize,
    board : Vec<Vec<Pos>>
}

impl Board {

    pub fn parse(rows : Vec<&str>) -> Board {
        let mut curr_row = 0;
        let mut curr_col = 0;
        let mut board = Vec::new();
        let num_cols = rows.get(1).unwrap().len();
        let num_rows = rows.len();
        for row in rows {
            let mut parsed_row = Vec::new();
            let chars = row.chars();
            for char in chars {
                let state = match char {
                    'L' => Empty,
                    '#' => Occupied,
                    '.' => Floor,
                    c   => panic!("Unexpected char found {}", c),
                };
                parsed_row.push(Pos{ row: curr_row, col: curr_col, state: RefCell::new(state)});
                curr_col += 1;
            }
            board.push(parsed_row);
            curr_col  = 0;
            curr_row += 1;
        }        
        Board {rows: num_rows, cols : num_cols, board: board}
    }

    pub fn eval(&self) -> i32 {
        let mut step_changes = -1;
        while step_changes != 0 {
            step_changes = self.step() as i32;            
        }
        self.num_occupied()
    }

    fn step(&self) ->  usize {
        let changes = self.collect_state_changes();        
        let num_changes = changes.len();
        apply_changes(changes);
        num_changes
    }

    fn num_occupied(&self) -> i32 {
        let mut count = 0;
        for row in &self.board {
            for pos in row {
                if pos.is_occupied() {
                    count += 1;
                }
            }
        }
        count
    }

    fn collect_state_changes(&self) -> Vec<(&Pos, Grid)> {
        let mut changes = Vec::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let pos = self.get(row as i32, col as i32).unwrap();                
                let adjacent = self.get_seen(pos.row as i32, pos.col as i32);
                let num_occupied = adj_occupied(adjacent);
                if pos.is_occupied() && num_occupied > 4 {
                    changes.push((pos, Empty));
                }
                if pos.is_empty() && num_occupied == 0 {
                    changes.push((pos, Occupied))
                }
            } 
        }
        changes
    }    

    fn get_adjacent(&self, row : i32, col : i32) -> Vec<&Pos> {
        let positions = vec![(row - 1, col - 1), (row - 1, col), (row - 1, col + 1), (row, col -1), (row, col + 1), (row + 1, col - 1), (row + 1, col), (row + 1, col + 1)];
        positions.iter().filter_map(|(r,c)| self.get(*r,*c)).collect()
    }

    fn get_seen(&self, row : i32, col : i32) -> Vec<&Pos> {
        let offsets = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        offsets.iter().filter_map(|tpl| self.get_along_line(row, col, *tpl)).collect()
    }

    fn get_along_line(&self, row: i32, col : i32, (r_off, c_off) : (i32,i32)) -> Option<&Pos>{
        let mut loop_counter = 1;
        loop {
            let m_pos = self.get(row + (r_off * loop_counter), col + (c_off * loop_counter));
            match m_pos {
                None => return None,
                Some(pos) => {
                    if ! pos.is_floor() {
                        return Some(pos);
                    }
                    loop_counter += 1;
                }
            }
        }   
        None
    }

    fn get(&self, row : i32, col: i32) -> Option<&Pos> {
        if row < 0 || row >= (self.rows as i32) || col < 0 || col >= (self.cols as i32) {
            None
        } else {
            Some(&self.board[row as usize][col as usize])
        }
    }    
}

fn adj_occupied(ps : Vec<&Pos>) -> usize {
    ps.iter().filter(|p| p.is_occupied() ).collect::<Vec<&&Pos>>().len()
}

fn apply_changes(changes : Vec<(&Pos, Grid)>) {
    changes.iter().for_each(|(pos, grd)| pos.set_state(*grd));
}

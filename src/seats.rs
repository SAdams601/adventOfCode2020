use std::slice::Iter;

pub fn calc_row_col(bsn : &String) -> (i32, i32) {
    let mut row_instr: Vec<char>= bsn.chars().take(7).collect();
    row_instr.reverse();
    let col_instr: Vec<char> = bsn.chars().rev().take(3).collect();
    (bin_chars_to_num('F','B', row_instr.iter()), bin_chars_to_num('L', 'R', col_instr.iter()))
}

fn bin_chars_to_num<'a,I>(zero : char, one : char, i: I) -> i32
where I : Iterator<Item = &'a char>
{
    let mut num : i32 = 0;
    for (idx, c) in i.enumerate() {
        match c {
            c if c == &zero => assert!(true),
            c if c == &one => num += 2_i32.pow(idx as u32),
            c => panic!("Unexpected row value: {}", c)
        }
    }
    return num;
}

pub fn calc_seat_id((row, col): (i32,i32)) -> i64 {
    ((row * 8) + col) as i64
}
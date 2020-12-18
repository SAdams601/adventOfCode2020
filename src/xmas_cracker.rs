use std::fs::read_to_string;

pub struct Xmas {
    data : Vec<i64>,
    preamble_len: usize
}

impl Xmas {

    pub fn new(data : Vec<i64>, len : usize) -> Xmas {
        Xmas { data, preamble_len: len }
    }

    pub fn find(&self) -> i64 {
        for (i, num) in self.data.iter().enumerate() {
            if i >= self.preamble_len {
                let factors = self.get_factors_for(i);
                let cln = factors.clone();
                let cannot_sum = factors.iter().all(|fc| {
                    let other_num = num - fc;
                    !cln.contains(&other_num)
                });
                if cannot_sum {
                    return *num;
                }
            }
        }
        -1
    }

    pub fn find_sum_range(&self, goal : i64) -> Vec<i64> {
        for i in 0..self.data.len() {
            let m_result = self.start_of_range(goal,i);
            if m_result.is_some() {
                return m_result.unwrap();
            }
        }
        Vec::new()
    }

    fn start_of_range(&self, goal : i64, index : usize) -> Option<Vec<i64>> {
        let mut sum = 0;
        for (i, num) in self.data[index..].iter().enumerate() {
            sum += num;
            if sum > goal {
                return None;
            } else if sum == goal {
                return Some(self.data[index..(index+i+1)].to_owned());
            }
        }
        None
    }

    fn get_factors_for(&self, i : usize) -> Vec<i64> {
        self.data[(i - self.preamble_len)..i].to_vec()
    }

}

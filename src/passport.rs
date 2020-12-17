use std::collections::HashMap;
use regex::{Regex, Captures, Match};

pub struct Passport {
    b_year:  Option<String>,
    i_year:  Option<String>,
    ex_year: Option<String>,
    height:  Option<String>,
    hair:    Option<String>,
    eye:     Option<String>,
    pid:     Option<String>,
    cid:     Option<String>,
}

impl Passport {

    pub fn is_valid(&self) -> bool {
        validate_num(1920, 2002, &self.b_year) &&
        validate_num(2010, 2020, &self.i_year) &&
        validate_num(2020, 2030, &self.ex_year) &&
        validate_height(&self.height) &&
        validate_hair(&self.hair) &&
        validate_eye(&self.eye) &&
        validate_pid(&self.pid)
    }
}

fn validate_num(min: i32, max: i32, m_num: &Option<String>) -> bool {
    match m_num {
        None => return false,
        Some(val) => {
            val.parse().map_or(false, |yr : i32| yr >= min && yr <= max)
        }
    }
}

fn validate_height(m_height: &Option<String>) -> bool {
    match m_height {
        None => false,
        Some(val) => {
            let re = Regex::new(r"(?P<h>\d+)(?P<unit>in|cm)").unwrap();
            re.captures(val).map_or(false,
                | mt | {
                    let h = mt.name("h").map(|m| m.as_str().to_string());
                    match mt.name("unit").unwrap().as_str() {
                        "cm" => validate_num(150,193,&h),
                        "in" => validate_num(59, 76, &h),
                        _ => false
                    }})

        }
    }
}

fn validate_hair(m_hair: &Option<String>) -> bool {
    m_hair.as_ref().map_or(false, |val| {
        let re = Regex::new(r"#[\da-f]{6}").unwrap();
        re.is_match(val.as_str())
    })
}

fn validate_eye(m_eye: &Option<String>) -> bool {
    let options = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    m_eye.as_ref().map_or(false, |val| options.contains(&val.trim()))
}

fn validate_pid(m_pid: &Option<String>) -> bool {
    m_pid.as_ref().map_or(false, |val| {
        Regex::new(r"\d{9}").unwrap().is_match(val.as_str())
    })
}


pub fn parse_passport(lines: &Vec<String>) -> Passport {
    let parts : Vec<&str> = lines.iter().map(|ln| ln.split_ascii_whitespace()).flatten().collect();
    let mut mp: HashMap<String, String> = HashMap::new();
    parts.iter().for_each(|part| {
        let split : Vec<String> = part.trim().split(":").map(str::to_string).collect();
        mp.insert(split[0].clone(), split[1].clone());
    });
    Passport{b_year : mp.remove("byr"),
        i_year: mp.remove("iyr"),
        ex_year: mp.remove("eyr"),
        height: mp.remove("hgt"),
        hair: mp.remove("hcl"),
        eye: mp.remove("ecl"),
        pid: mp.remove("pid"),
        cid: mp.remove("cid")}
}
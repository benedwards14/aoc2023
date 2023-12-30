use std::{fs::read_to_string, collections::HashMap};
use std::cell::RefCell;

#[derive(PartialEq)]
enum Status {
    OPERATIONAL,
    DAMAGED,
    UNKNOWN  
}

impl Status {
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::OPERATIONAL,
            '#' => Self::DAMAGED,
            '?' => Self::UNKNOWN,
            _ => panic!("Oh no!")
        }
    }
}

struct Row {
    pattern: Vec<Status>,
    pattern_len: usize,
    damaged: Vec<usize>,
    damaged_len: usize,
    cache: RefCell<HashMap<(usize, usize), i64>>
}

fn parse_input(repeat: i32) -> Vec<Row> {
    let mut rows = Vec::new();

    for row in read_to_string("./data.txt").unwrap().lines() {
        let mut parsed_pattern = Vec::new();
        let mut parsed_damage = Vec::new();
        for idx in (0..repeat).into_iter() {
            let (pattern, damaged) = row.split_once(" ").unwrap();
            parsed_pattern.extend(pattern.chars().map(|p| Status::parse(p)));
            parsed_damage.extend(damaged.trim().split(",").map(|d| d.parse::<usize>().unwrap()));

            if idx != repeat -1 {
                parsed_pattern.push(Status::UNKNOWN);
            }
        }
        let parsed_pattern_len = parsed_pattern.len();
        let parsed_damage_len = parsed_damage.len();
        

        rows.push(Row {
            pattern: parsed_pattern, 
            pattern_len: parsed_pattern_len,
            damaged: parsed_damage,
            damaged_len: parsed_damage_len,
            cache: RefCell::new(HashMap::new())
        });
    }

    rows
}

impl Row {
    fn find_combinations(&self) -> i64 {
        self.find_combinations_recursive(0, 0)
    } 

    fn if_can_fit_damage(&self, pattern_idx: usize, damage_idx: usize) -> bool {
        let Some(damage_count) = self.damaged.get(damage_idx) else {return false;};
        
        for idx in (0..*damage_count).into_iter() {
            match self.pattern.get(pattern_idx + idx) {
                None | Some(Status::OPERATIONAL) => { return false; }
                Some(Status::DAMAGED) | Some(Status::UNKNOWN) => {}
            }
        }

        match self.pattern.get(pattern_idx + *damage_count) {
            None | Some(Status::OPERATIONAL) | Some(Status::UNKNOWN) => true,
            Some(Status::DAMAGED)  => false     
        }
    }

    fn find_combinations_recursive(&self, mut pattern_idx: usize, mut damage_idx: usize) -> i64 {
        if let Some(cached) = self.cache.borrow().get(&(pattern_idx, damage_idx)) {
            return *cached;
        }

        while let Some(Status::OPERATIONAL) = self.pattern.get(pattern_idx) {
            pattern_idx += 1;
        }

        if pattern_idx >= self.pattern_len && damage_idx == self.damaged_len {
            return 1;
        } else if pattern_idx >= self.pattern_len {
            return 0;
        }

        let mut combinations = 0;

        if self.pattern[pattern_idx] == Status::UNKNOWN {
            combinations += self.find_combinations_recursive(pattern_idx + 1, damage_idx);
        }

        let can_fit_damage = self.if_can_fit_damage(pattern_idx, damage_idx);
        if can_fit_damage {
            combinations += self.find_combinations_recursive(
                pattern_idx + self.damaged[damage_idx] + 1, 
                damage_idx + 1
            );
        }
        
        self.cache.borrow_mut().insert((pattern_idx, damage_idx), combinations);
        combinations
    }
    
}

fn main() {
    let input1 = parse_input(1);
    let input2 = parse_input(5);

    let all_combs1: i64 = input1.iter().map(|r| r.find_combinations()).sum();
    println!("{}", all_combs1); // 7541

    let all_combs2: i64 = input2.iter().map(|r| r.find_combinations()).sum();
    println!("{}", all_combs2);
}

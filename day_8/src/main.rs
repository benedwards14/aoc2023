use std::{fs::read_to_string, collections::{HashMap, HashSet}};
use regex::Regex;

#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT
}

impl Direction {
    fn parse(line: &str) -> Vec<Self> {
        line.chars().map(
            |c| match c {
                'L' => Self::LEFT,
                'R' => Self::RIGHT,
                _ => panic!("Oh No!")
            }
        ).collect()
    }
}

struct Map {
    directions: Vec<Direction>,
    moves: HashMap<String, (String, String)>
}

fn parse_input() -> Map {
    let mut moves = HashMap::new();
    let input_str = read_to_string("./data.txt").unwrap();
    let mut lines = input_str.lines();
    let directions = Direction::parse(lines.next().unwrap());

    assert!(lines.next().unwrap().is_empty());
    
    let re = Regex::new(r"^([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)$").unwrap();
    for line in lines {
        let Some(capture) = re.captures(line) else { panic!("Oh No!") };
        let (_, [initial, left, right]) = capture.extract();
        moves.insert(initial.to_owned(), (left.to_owned(), right.to_owned()));
    }

    Map { directions, moves }
}

struct Step<'a> {
    steps_to_end: HashSet<i32>,
    next: &'a str
}

impl Map {
    fn walk_one<'a>(&'a self, start: &'a str) -> Step {
        let mut curr = start;
        let mut step: i32 = 0;
        let mut steps_to_end = HashSet::new();
        for next_direction in &self.directions {
            if curr.chars().last().unwrap() == 'Z' { 
                steps_to_end.insert(step);
            }

            let (next_left, next_right) = self.moves.get(curr).unwrap();

            curr = match next_direction {
                Direction::LEFT => next_left,
                Direction::RIGHT => next_right
            };

            step += 1;
        }

        Step { steps_to_end, next: curr }
        
    }

    fn walk_to_zzz(&self) -> usize {
        let mut curr = "AAA";
        let mut step = 0;
        let direction_len = self.directions.len();
    
        loop {
            if curr == "ZZZ" { break; }
    
            let next_direction = &self.directions[step % direction_len];
            let (next_left, next_right) = self.moves.get(curr).unwrap();
    
            curr = match next_direction {
                Direction::LEFT => next_left,
                Direction::RIGHT => next_right
            };
    
            step += 1;
        }
    
        step
    }

    fn get_combined_moves(&self) -> HashMap<&str, Step> {
        let mut combined_moves = HashMap::new();
        self.moves.keys().for_each(
            | start | 
            { combined_moves.insert(&start[..], self.walk_one(start)); }
        );

        combined_moves
    }
}


fn walk_to_end(start: &str, moves: &HashMap<&str, Step>) -> i64 {
    let mut curr = start;
    let mut count = 0;

    loop {
        curr = moves.get(curr).unwrap().next;
        count += 1;

        if curr.chars().last().unwrap() == 'Z' {
            break;
        }
    }

    count
}


fn main() {
    let map = parse_input();

    println!("{}", map.walk_to_zzz()); //17873

    let combined_moves = map.get_combined_moves();

    let starts: Vec<&String> = map.moves.keys().filter(|k| k.chars().last().unwrap() == 'A').collect();

    let number_of_loops: i64 = starts.iter().map(|s| walk_to_end(s, &combined_moves)).product();
    let step_length: i64 = map.directions.len().try_into().unwrap();

    println!("{}", number_of_loops * step_length);
}

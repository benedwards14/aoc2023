use std::{fs::read_to_string, collections::HashMap};

fn parse_input() -> Vec<String> {
    read_to_string("./data.txt").unwrap().split(",").map(|s| s.to_string()).collect()
}

fn get_hash(s: &str) -> u32 {
    let mut value = 0;

    for c in s.chars() {
        let ascii_value = c as u32;
        value += ascii_value;
        value *= 17;
        value %= 256;
    }

    value
}

#[derive(Debug)]
enum Instruction {
    Add(String, u32),
    Remove(String)
}

impl Instruction {
    fn parse(input: &str) -> Self {
        if let Some((label, value)) = input.split_once("=") {
            return Self::Add(label.to_string(), value.parse().unwrap());
        } else if let Some((label, _)) = input.split_once("-") {
            return Self::Remove(label.to_string());
        } else {
            panic!("Oh No!")
        }
    }
    
}

fn parse(input: &Vec<String>) -> Vec<Instruction> {
    input.iter().map(|i| Instruction::parse(i)).collect()
}

fn process(instructions: &Vec<Instruction>) -> u32 {
    let mut boxes: HashMap<u32, Vec<(String, u32)>> = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Remove(label) => {
                let hash = get_hash(label);
                if let Some(b) = boxes.get_mut(&hash) {
                    *b = (*b).iter()
                        .filter(|l| (*l).0 != *label)
                        .map(|l| l.to_owned())
                        .collect();
                }
            },
            Instruction::Add(label, value) => {
                let hash = get_hash(label);
                if let Some(b) = boxes.get_mut(&hash) {
                    let mut updated = false;
                    *b = (*b).iter()
                        .map(|l| {
                            if l.0 == *label {
                                updated = true;
                                (l.0.to_owned(), *value)
                            } else {
                                l.to_owned()
                            }
                        })
                        .collect();

                    if !updated {
                        (*b).push((label.to_owned(), *value));
                    }
                } else {
                    boxes.insert(hash, Vec::from([(label.to_owned(), *value)]));
                }
            }
            
        }
    }

    let mut total = 0;
    for (box_idx, lenses) in boxes {
        let box_value = box_idx + 1;
        for (lens_idx, (_, value))  in lenses.iter().enumerate() {
            let lens_value: u32 = (lens_idx + 1).try_into().unwrap();
            total += box_value * lens_value * value;
        }
    }

    total
}

fn main() {
    let input = parse_input();

    let sum: u32 = input.iter().map(|s| get_hash(s)).sum();
    println!("{}", sum);
    let parsed = parse(&input);
    println!("{}", process(&parsed));
    
}

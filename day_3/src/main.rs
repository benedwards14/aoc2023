
use std::fs::read_to_string;

struct Number {
    value: u32,
    row_idx: usize,
    start_idx: usize,
    end_idx: usize
}

struct Part {
    value: char,
    row_idx: usize,
    column_idx: usize
}

struct Schematic {
    numbers: Vec<Number>,
    parts: Vec<Part>
}

fn parse_input() -> Schematic {
    let mut numbers: Vec<Number> = Vec::new();
    let mut parts: Vec<Part> = Vec::new();

    for (row_idx, line) in read_to_string("./data.txt").unwrap().lines().enumerate() {
        let mut number_part = 0;
        let mut number_len = 0;
        let mut last_number_idx = 0;
        let line_length = line.len();

        for (column_idx, character) in line.chars().enumerate() {
            let mut number_ready = false;

            match character.to_digit(10) {
                Some(x) => {
                    number_part = number_part * 10 + x;
                    number_len += 1;
                    last_number_idx = column_idx;
                    if column_idx == line_length - 1 {
                        number_ready = true
                    }
                },
                None => { number_ready = number_len > 0; }
            }
            if number_ready {
                numbers.push(Number{
                    value: number_part,
                    row_idx,
                    start_idx: last_number_idx + 1 - number_len,
                    end_idx: last_number_idx
                });

                number_part = 0;
                number_len = 0;
            }

            match character {
                '.' => {},
                '0'..='9' => {},
                x => parts.push(Part{
                    value: x, row_idx, column_idx
                })
            }
        }
    }

    Schematic {
        numbers, parts
    }
}

impl Number {
    fn is_adjacent(&self, part: &Part) -> bool {
        let is_adjacent_row = self.row_idx >= part.row_idx - 1 && self.row_idx <= part.row_idx + 1;
        let is_adjacent_column = self.start_idx <= part.column_idx + 1 && self.end_idx >= part.column_idx - 1;

        is_adjacent_row && is_adjacent_column
    }

    fn has_adjacent_part(&self, parts: &Vec<Part>) -> bool {
        parts.iter().any(|p| self.is_adjacent(p))
    } 
    
}

impl Part {
    fn get_adjacent_numbers<'a>(&'a self, numbers: &'a Vec<Number>) -> Vec<&Number> {
        numbers.iter().filter(|n| n.is_adjacent(self)).collect()
    }
}

fn main() {
    let input = parse_input();

    let total:u32 = input.numbers.iter().filter(|n| n.has_adjacent_part(&input.parts)).map(|n| n.value).sum();

    println!("{}", total);

    let potential_gears: Vec<&Part> = input.parts.iter().filter(|p| p.value == '*').collect();

    let total2: u32 = potential_gears.iter().map(
        |g| {
            let adjacent = g.get_adjacent_numbers(&input.numbers);

            match adjacent[..] {
                [a,b] => a.value * b.value,
                _ => 0
            }
        }
    ).sum();

    println!("{}", total2);

    // let number  = 220;
    // let part =2;
    // println!("{} {}", input.parts[part].row_idx, input.parts[part].column_idx);
    // println!("{} {} {}", input.numbers[number].value, input.numbers[number].start_idx, input.numbers[number].end_idx);
    // println!("{}", input.numbers[number].has_adjacent_part(&input.parts));

    // println!("{}", input.numbers.iter().enumerate().filter(|x| x.1.value == 455).map(|x| x.0).collect::<Vec<_>>()[1])
}

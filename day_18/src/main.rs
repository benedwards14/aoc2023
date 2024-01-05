use std::{fs::read_to_string, collections::{HashSet}};

fn parse_input() -> Vec<Instruction>  {
    let mut instructions = Vec::new();
    for row in read_to_string("./data.txt").unwrap().lines() {
        let split = row.split_whitespace().collect::<Vec<&str>>();

        instructions.push(Instruction { 
            direction: Direction::parse(split[0]), 
            count: split[1].parse().unwrap(),
            colour: split[2].to_owned()
        });
    }

    instructions
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Instruction {
    direction: Direction,
    count: i64,
    colour: String
}

#[derive(Debug)]
struct VerticalEdge {
    direction: Direction,
    col_idx: i64,
    start_row: i64,
    end_row: i64
}

impl Direction {
    fn parse(line: &str) -> Self {
        match line {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unreachable!()            
        }
    }

    fn step(&self, current: &(i64, i64), count: i64) -> (i64, i64) {
        let (row, col) = *current;
        match self {
            Self::Up => (row - count, col),
            Self::Down => (row + count, col),
            Self::Left => (row, col - count),
            Self::Right => (row, col + count),
        }
    }
}

impl VerticalEdge {
    fn is_corner(&self, point: i64) -> bool {
        point == self.start_row || point == self.end_row
    }
}

fn get_hexadecimal_instr(instr: &Instruction) -> Instruction {
    let hex = instr.colour.trim_end_matches(')').trim_start_matches(['(', '#']);

    let (count_str, dir_str) = hex.split_at(5);
    let direction = match dir_str {
        "3" => Direction::Up,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "0" => Direction::Right,
        _ => unreachable!()            
    };

    let count = i64::from_str_radix(count_str, 16).unwrap();

    Instruction { direction, count, colour: String::new() }
}

fn get_vertical_edges(instructions: &Vec<Instruction>) -> Vec<VerticalEdge> {
    let mut curr = (0,0);
    let mut edges = vec![];

    for instr in instructions {
        let (row_1, col_1) = curr;
        let (row_2, col_2) = instr.direction.step(&curr, instr.count);
        match instr.direction {
            Direction::Up => { edges.push(VerticalEdge { direction: Direction::Up, col_idx: col_1, start_row: row_2, end_row: row_1 }) },
            Direction::Down => { edges.push(VerticalEdge { direction: Direction::Down, col_idx: col_1, start_row: row_1, end_row: row_2 }) },
            _ => {}
        }
        
        curr=(row_2, col_2);
    }

    edges
}

fn get_partitions(edges: &Vec<VerticalEdge>) -> Vec<(i64, i64)> {
    let all_start_rows: HashSet<i64> = edges.iter().map(|e| e.start_row).collect();
    let all_end_rows: HashSet<i64> = edges.iter().map(|e| e.end_row).collect();
    let mut all_corner_rows: Vec<i64> = all_start_rows.union(&all_end_rows).map(|i| *i).collect();
    all_corner_rows.sort();

    let mut partitions = Vec::new();

    let mut last_corner_row = 0;
    for (i, corner_row) in all_corner_rows.iter().enumerate() {
        if i == 0 {
            partitions.push((*corner_row, *corner_row));
            last_corner_row = *corner_row;
        } else {
            if corner_row - 1 != last_corner_row {
                partitions.push((last_corner_row + 1, corner_row - 1));
            }
    
            partitions.push((*corner_row, *corner_row));
            last_corner_row = *corner_row;
        }
    }

    partitions
}


fn count(edges: &Vec<VerticalEdge>) -> i64 {
    let mut total = 0;

    for (start, end) in get_partitions(edges) {
        let mut relevant_edges: Vec<&VerticalEdge> = edges.iter()
            .filter(|e| e.start_row <= start && start <= e.end_row)
            .collect();
        relevant_edges.sort_by_key(|k| k.col_idx);

        let mut count = 1;
        let mut last_edge = relevant_edges[0];
        let mut inside = !last_edge.is_corner(start);
        let mut on_corner = last_edge.is_corner(start);

        for edge in relevant_edges[1..].iter() {
            count += 1;

            if inside || on_corner {
                count += edge.col_idx - last_edge.col_idx -1;
            }

            if !edge.is_corner(start) {
                assert!(!on_corner);
                inside  = !inside;
            } else {
                if on_corner && last_edge.direction == edge.direction {
                    inside = !inside;
                }
                on_corner = !on_corner;
            }
            last_edge = edge;
        }
        total += count * (end - start + 1);
    }

    total
}



fn main() {
    let instructions = parse_input();
    let hex_instructions = instructions.iter().map(|a| get_hexadecimal_instr(a)).collect();

    let vertical_edges = get_vertical_edges(&instructions);
    let hex_vertical_edges = get_vertical_edges(&hex_instructions);

    println!("{}", count(&vertical_edges));
    println!("{}", count(&hex_vertical_edges));

}

use std::{fs::read_to_string, collections::{HashMap, VecDeque}};

const MAX_ROW: i32 = 140;
const MAX_COLUMN: i32 = 140;

fn parse_input() -> ((i32, i32), HashMap<(i32, i32), Pipe>) {
    let mut pipes = HashMap::new();
    let input_str = read_to_string("./data.txt").unwrap();
    let lines = input_str.lines();
    let mut start: (i32, i32) = (0,0);

    for (idx, row) in lines.enumerate() {
        let row_idx: i32 = idx.try_into().unwrap();
        for (idx, loc) in row.chars().enumerate() {
            let column_idx = idx.try_into().unwrap();
            if loc == 'S' {
                start = (row_idx, column_idx);
            }

            if let Some(pipe) = Pipe::parse(&loc) {
                pipes.insert((row_idx, column_idx), pipe);
            }
        }
    }

    (start, pipes)
}

#[derive(Debug, PartialEq)]
enum Pipe {
    VERTICAL,
    HORIZONTAL,
    NORTH_EAST,
    NORTH_WEST,
    SOUTH_EAST,
    SOUTH_WEST,
}

impl Pipe {
    fn parse(symbol: &char) -> Option<Self> {
        match symbol {
            '|' => Some(Self::VERTICAL),
            '-' => Some(Self::HORIZONTAL),
            'L' => Some(Self::NORTH_EAST),
            'J' => Some(Self::NORTH_WEST),
            '7' => Some(Self::SOUTH_WEST),
            'F' => Some(Self::SOUTH_EAST),
            'S' => Some(Self::SOUTH_WEST),
            _ => None
        }
    }

    fn connect_to(&self, start: &(i32, i32)) -> ((i32, i32), (i32, i32)) {
        let (row, column) = *start;
        match self {
            Self::VERTICAL   => ((row + 1, column), (row - 1, column)),
            Self::HORIZONTAL => ((row, column + 1), (row, column - 1)),
            Self::NORTH_EAST => ((row - 1, column), (row, column + 1)),
            Self::NORTH_WEST => ((row - 1, column), (row, column - 1)),
            Self::SOUTH_WEST => ((row + 1, column), (row, column - 1)),
            Self::SOUTH_EAST => ((row + 1, column), (row, column + 1)),
        }
    }
}

fn walk(start: (i32, i32), pipes: &HashMap<(i32, i32), Pipe>) -> HashMap<(i32, i32), i32> {
    let mut to_do = VecDeque::from([(start, 0)]);
    let mut distances = HashMap::from([(start, 0)]);

    loop {
        let Some((location, distance)) = to_do.pop_front() else { break; };
        let pipe = pipes.get(&location).unwrap();

        let (connects_1, connects_2) = pipe.connect_to(&location);

        if pipes.contains_key(&connects_1) && !distances.contains_key(&connects_1) {
            distances.insert(connects_1, distance + 1);
            to_do.push_back((connects_1, distance + 1));
        }

        if pipes.contains_key(&connects_2) && !distances.contains_key(&connects_2) {
            distances.insert(connects_2, distance + 1);
            to_do.push_back((connects_2, distance + 1));
        }

    }

    distances
}

fn find_enclosing_space(pipes: HashMap<(i32, i32), &Pipe>) -> i32 {
    let mut number_of_spaces = 0;

    for row_idx in (0..MAX_ROW).into_iter() {
        let mut in_loop = false;
        let mut wall_start = None;

        for column_idx in (0..MAX_COLUMN).into_iter() {

            if let Some(pipe) = pipes.get(&(row_idx, column_idx)) {
                match pipe {
                    Pipe::VERTICAL => { in_loop = !in_loop; },
                    Pipe::HORIZONTAL => { assert!(wall_start != None); },
                    Pipe::NORTH_EAST => {
                        assert!(wall_start == None);
                        wall_start = Some(pipe); 
                    },
                    Pipe::SOUTH_EAST => {
                        assert!(wall_start == None);
                        wall_start = Some(pipe); 
                    },
                    Pipe::NORTH_WEST => {
                        match wall_start {
                            Some(Pipe::SOUTH_EAST) => {
                                wall_start = None;
                                in_loop = !in_loop;
                            },
                            Some(Pipe::NORTH_EAST) => { wall_start = None; },
                            _ => panic!("Oh No!")
                        }
                    },
                    Pipe::SOUTH_WEST => {
                        match wall_start {
                            Some(Pipe::NORTH_EAST) => {
                                wall_start = None;
                                in_loop = !in_loop;
                            },
                            Some(Pipe::SOUTH_EAST) => { wall_start = None; },
                            _ => panic!("Oh No!")
                        }
                    },
                }

            } else {
                assert!(wall_start == None);
                if in_loop {
                    number_of_spaces += 1;
                }
            }
        }
    }

    number_of_spaces
}

fn main() {
    let (start, pipes) = parse_input();

    let distances =  walk(start, &pipes);
    let largest_distance = distances.values().max().unwrap();
    println!("{}", largest_distance);
    let filtered_pipes = distances.keys().map(|p| (*p, pipes.get(p).unwrap())).collect::<HashMap<_, _>>();
    let number_of_spaces = find_enclosing_space(filtered_pipes);
    println!("{}", number_of_spaces);
}

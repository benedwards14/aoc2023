use std::{fs::read_to_string, collections::{HashMap, HashSet}};

const SIZE: usize = 110;

#[derive(Clone, Debug)]
enum Direction {
    Up(Option<(usize, usize)>),
    Down(Option<(usize, usize)>),
    Left(Option<(usize, usize)>),
    Right(Option<(usize, usize)>)
}

#[derive(Debug)]
struct Point {
    going_up: Vec<Direction>,
    going_down: Vec<Direction>,
    going_left: Vec<Direction>,
    going_right: Vec<Direction>
}

fn parse_input() -> HashMap<(usize, usize), Point>  {
    let mut mirrors = HashMap::new();
    for (row_idx, row) in read_to_string("./data.txt").unwrap().lines().enumerate() {
        for (column_idx, space) in row.chars().enumerate() {
            match space {
                '.' => {},
                a => { mirrors.insert((row_idx, column_idx), a); }
            }
        }
    }

    let points = parse_points(mirrors);

    points
}

fn parse_points(mirrors: HashMap<(usize, usize), char>) -> HashMap<(usize, usize), Point> {
    let mut points = HashMap::new();

    for ((rows_idx, column_idx), mirror) in &mirrors {
        let mut mirror_above = Direction::Up(None);
        for idx in (1..SIZE).into_iter() {
            if *rows_idx < idx { break; }
            let point = (rows_idx - idx, *column_idx);
            if mirrors.contains_key(&point) { 
                mirror_above = Direction::Up(Some(point));
                break;
            }
        }

        let mut mirror_below = Direction::Down(None);
        for idx in (1..SIZE).into_iter() {
            if rows_idx + idx == SIZE { break; }
            let point = (rows_idx + idx, *column_idx);
            if mirrors.contains_key(&point) { 
                mirror_below = Direction::Down(Some(point));
                break;
            }
        }

        let mut mirror_left = Direction::Left(None);
        for idx in (1..SIZE).into_iter() {
            if *column_idx < idx { break; }
            let point = (*rows_idx, column_idx - idx);
            if mirrors.contains_key(&point) { 
                mirror_left = Direction::Left(Some(point));
                break;
            }
        }

        let mut mirror_right = Direction::Right(None);
        for idx in (1..SIZE).into_iter() {
            if column_idx + idx == SIZE { break; }
            let point = (*rows_idx, column_idx + idx);
            if mirrors.contains_key(&point) { 
                mirror_right = Direction::Right(Some(point));
                break;
            }
        }

        let point = match mirror {
            '/' => {
                Point { 
                    going_up: vec![mirror_right],
                    going_down: vec![mirror_left],
                    going_left: vec![mirror_below],
                    going_right: vec![mirror_above]
                }
            },
            '\\' => {
                Point { 
                    going_up: vec![mirror_left],
                    going_down:vec![mirror_right],
                    going_left:vec![mirror_above],
                    going_right:vec![mirror_below]
                }
            },
            '|' => {
                Point { 
                    going_up: vec![mirror_above.clone()],
                    going_down:vec![mirror_below.clone()],
                    going_left:vec![mirror_above.clone(), mirror_below.clone()],
                    going_right:vec![mirror_above, mirror_below]
                }
            },
            '-' => {
                Point { 
                    going_up: vec![mirror_left.clone(), mirror_right.clone()],
                    going_down:vec![mirror_left.clone(), mirror_right.clone()],
                    going_left:vec![mirror_left],
                    going_right:vec![mirror_right]
                }
            },
            _ => panic!("Oh No!")
        };

        points.insert((*rows_idx, *column_idx), point);
    }

    points
}

impl Direction {
    fn extract(&self) -> Option<(usize, usize)> {
        match self {
            Direction::Up(p) => p.to_owned(),
            Direction::Down(p) => p.to_owned(),
            Direction::Left(p) => p.to_owned(),
            Direction::Right(p) => p.to_owned(),
        }
    }


    fn get_edge(&self, from: &(usize, usize)) -> (usize, usize) {
        let (r,c) = from;
        match self {
            Direction::Up(None) => (0, *c),
            Direction::Down(None) => (SIZE -1 , *c),
            Direction::Left(None) => (*r, 0),
            Direction::Right(None) => (*r, SIZE - 1),
            _ => panic!("")
        }
    }
}

fn walk(start: Direction, points: &HashMap<(usize, usize), Point>) -> HashSet<((usize, usize), (usize, usize))> {
    let mut to_go = Vec::from([start]);
    let mut seen = HashSet::new();

    while let Some(current) = to_go.pop() {
        let next_steps = match &current {
            Direction::Up(Some(point)) => &points.get(point).unwrap().going_up,
            Direction::Down(Some(point)) => &points.get(point).unwrap().going_down,
            Direction::Left(Some(point)) => &points.get(point).unwrap().going_left,
            Direction::Right(Some(point)) => &points.get(point).unwrap().going_right,
            _ => panic!("Oh No!")
        };

        let Some(current_coord) = current.extract() else { panic!(""); };

        for next in next_steps {
            if let Some(next_coord) = next.extract() {
                let direction = (current_coord.to_owned(), next_coord);
                if !seen.contains(&direction) {
                    seen.insert(direction);
                    to_go.push(next.to_owned());
                }

            } else {
                let edge = next.get_edge(&current_coord);
                seen.insert((current_coord.to_owned(), edge));
            }
        }
    }

    seen
}


fn get_first_mirror(start: Direction, points: &HashMap<(usize, usize), Point>) -> (Direction, HashSet<(usize, usize)>){
    let mut curr = start;
    let mut seen = HashSet::new();
    loop {
        let coords = curr.extract().unwrap();

        if points.contains_key(&coords) {
            break;
        }

        curr = match curr {
            Direction::Up(Some((r,c))) => Direction::Up(Some((r-1,c))),  
            Direction::Down(Some((r,c))) => Direction::Down(Some((r+1,c))), 
            Direction::Left(Some((r,c))) => Direction::Left(Some((r,c-1))), 
            Direction::Right(Some((r,c))) => Direction::Right(Some((r,c+1))), 
            _ => panic!("Oh No!")
        };

        seen.insert(coords);
    }

    (curr, seen)
}


fn count_energized(start: Direction, points: &HashMap<(usize, usize), Point>) -> usize {
    let (first_mirror, mut energized) = get_first_mirror(start, points);
    let walked_to = walk(first_mirror, points);

    for ((start_row, start_col), (end_row, end_col)) in walked_to.iter() {
        if *start_row == *end_row {
            let(start, end) = if *start_col > *end_col {
                (*end_col, *start_col)
            } else {
                (*start_col, *end_col)
            };
            for column_idx in (start..=end).into_iter() {
                energized.insert((*start_row, column_idx));
            }
        } else if *start_col == *end_col {
            let (start,end) = if *start_row > *end_row {
                (*end_row, *start_row)
            } else {
                (*start_row, *end_row)
            };
            for row_idx in (start..=end).into_iter() {
                energized.insert((row_idx, *start_col));
            }
        } else {
            panic!("Oh No!")
        }
    }

    energized.len()
}


fn get_all_starts() -> Vec<Direction> {
    let mut starts = Vec::new();

    for idx in 0..SIZE {
        starts.push(Direction::Up(Some((SIZE - 1, idx))));
        starts.push(Direction::Down(Some((0, idx))));
        starts.push(Direction::Left(Some((idx, SIZE - 1))));
        starts.push(Direction::Right(Some((idx, 0))));
    }

    starts
}


fn main() {
    let points = parse_input();

    println!("{}", count_energized(Direction::Right(Some((0,0))), &points)); // 7199

    let start_points = get_all_starts();

    let max = start_points.iter().map(|s| count_energized(s.to_owned(), &points)).max().unwrap();
    println!("{}", max); // 7438
}



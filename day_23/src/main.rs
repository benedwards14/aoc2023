use std::{fs::read_to_string, collections::{HashMap, VecDeque, HashSet}};

const SIZE: i32 = 141;

fn parse_input() -> HashMap<(i32, i32), String> {
    let mut path = HashMap::new();

    for (row_idx, row) in read_to_string("./data.txt").unwrap().lines().enumerate() {
        for (column_idx, space) in row.chars().enumerate() {
            if space == '#' { continue; }
            path.insert((row_idx.try_into().unwrap(), column_idx.try_into().unwrap()), space.to_string());
        }
        
    }

    path
}

#[derive(Debug)]
struct Path {
    location: (i32, i32),
    steps: i32,
    is_uphill: bool
}

#[derive(Debug)]
struct Junction {
    endpoints: Vec<Path>
}

fn is_end(coords: &(i32, i32)) -> bool {
    coords.0 == SIZE - 1
}

fn parse_path(start: (i32, i32), paths: &HashMap<(i32, i32), String>) -> Junction {
    let start_space = paths.get(&start).unwrap();
    assert!(start_space == ".");

    let mut to_do = VecDeque::from([(start.clone(), 0, false)]);
    let mut endpoints = Vec::new();
    let mut visited = HashSet::new();

    while let Some((current, curr_steps, is_uphill)) = to_do.pop_front()  {
        let current_path = paths.get(&current).unwrap();
        let (curr_row, curr_col) = current;

        let next_steps: Vec<(i32, i32)> = vec![(curr_row + 1, curr_col), (curr_row -1 , curr_col), (curr_row, curr_col+1), (curr_row, curr_col-1)]
            .iter()
            .filter(|n| paths.contains_key(n) && !visited.contains(*n))
            .map(|n| n.to_owned())
            .collect();

        if is_end(&current) || (curr_steps > 0 && next_steps.len() > 1) {
            assert!(current_path == ".");
            endpoints.push(Path { location: current, steps: curr_steps, is_uphill });
            continue;
        }

        visited.insert(current.clone());

        for next in next_steps {
            let next_is_uphill = is_uphill || (next.0 < curr_row && current_path == "v") || (next.1 < curr_col && current_path == ">");
            to_do.push_back((next, curr_steps + 1, next_is_uphill))
        }
    }

    Junction { endpoints }
}

fn parse_all_paths(paths: &HashMap<(i32, i32), String>) -> HashMap<(i32, i32), Junction> {
    let mut to_do = vec![(0,1)];
    let mut junctions = HashMap::new();

    while let Some(next) = to_do.pop() {
        let junction = parse_path(next, paths);
        let next_start_points: Vec<(i32, i32)> = junction.endpoints
            .iter()
            .map(|e| e.location.to_owned())
            .filter(|e| !is_end(e) && !junctions.contains_key(e))
            .collect();
        to_do.extend(next_start_points);
        junctions.insert(next, junction);
    }

    junctions
}

fn get_longest_path(start: (i32, i32), junctions: &HashMap<(i32, i32), Junction>, allow_uphill: bool) -> i32 {
    let mut to_do = vec![(start,0)];
    let mut visited = HashSet::new();
    let mut found_paths = Vec::new();

    while let Some((next, current_length)) = to_do.pop() {
        if is_end(&next) {
            found_paths.push(current_length);
            continue;
        }

        if visited.contains(&next) {
            visited.remove(&next);
            continue;
        }
        visited.insert(next);
        to_do.push((next, current_length));

        let next_junctions = &junctions.get(&next).unwrap().endpoints;
        for next in next_junctions {
            if visited.contains(&next.location) || (!allow_uphill && next.is_uphill) {
                continue;
            }

            to_do.push((next.location, current_length + next.steps));
        }
    }

    *found_paths.iter().max().unwrap()
}

fn main() {
    let paths = parse_input();
    let junctions = parse_all_paths(&paths);

    println!("{}", get_longest_path((0,1), &junctions, false)); //2206
    println!("{}", get_longest_path((0,1), &junctions, true)); //6490
}

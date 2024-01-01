use std::{fs::read_to_string, collections::{HashSet, HashMap}};

const MAX_ROW: usize = 100;
const MAX_COLUMN: usize = 100;

fn parse_input() -> (HashSet<(usize, usize)>, HashSet<(usize, usize)>) {
    let mut rocks = HashSet::new();
    let mut boulders = HashSet::new();

    for (row_idx, row) in read_to_string("./data.txt").unwrap().lines().enumerate() {
        for (column_idx, column) in row.chars().enumerate() {
            match column {
                'O' => { boulders.insert((row_idx, column_idx)); },
                '#' => { rocks.insert((row_idx, column_idx)); },
                _ => { assert!(column == '.'); }
                
            }
        }
    }

    (rocks, boulders)
}

fn tilt_north(rocks: &HashSet<(usize, usize)>, boulders: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut new_boulders = HashSet::new();
    for column_idx in (0..MAX_COLUMN).into_iter() {
        let mut next_available: (usize, usize) = (0,column_idx);

        for row_idx in (0..MAX_ROW).into_iter() {
            let point = (row_idx, column_idx);
            if rocks.contains(&point) {
                next_available = (row_idx + 1, column_idx);
            }

            if boulders.contains(&point) {
                new_boulders.insert(next_available);
                next_available = (next_available.0 + 1, column_idx);
            }
        }
    }

    new_boulders
}

fn tilt_south(rocks: &HashSet<(usize, usize)>, boulders: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut new_boulders = HashSet::new();
    for column_idx in (0..MAX_COLUMN).into_iter() {
        let mut next_available: (usize, usize) = (MAX_ROW - 1,column_idx);

        for idx in (0..MAX_ROW).into_iter() {
            let row_idx = MAX_ROW - idx - 1;
            let point = (row_idx, column_idx);
            if rocks.contains(&point) && row_idx !=0 {
                next_available = (row_idx - 1, column_idx);
            }

            if boulders.contains(&point) {
                new_boulders.insert(next_available);
                if row_idx != 0 {
                    next_available = (next_available.0 - 1, column_idx);
                }
            }
        }
    }

    new_boulders
}

fn tilt_east(rocks: &HashSet<(usize, usize)>, boulders: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut new_boulders = HashSet::new();
    for row_idx in (0..MAX_ROW).into_iter() {
        let mut next_available: (usize, usize) = (row_idx, MAX_COLUMN - 1);

        for idx in (0..MAX_COLUMN).into_iter() {
            let column_idx = MAX_COLUMN - idx - 1;
            let point = (row_idx, column_idx);
            if rocks.contains(&point) && column_idx !=0 {
                next_available = (row_idx, column_idx - 1);
            }

            if boulders.contains(&point) {
                new_boulders.insert(next_available);
            if column_idx != 0 {
                next_available = (row_idx, next_available.1 - 1);
            }
            }
        }
    }

    new_boulders
}

fn tilt_west(rocks: &HashSet<(usize, usize)>, boulders: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut new_boulders = HashSet::new();
    for row_idx in (0..MAX_ROW).into_iter() {
        let mut next_available: (usize, usize) = (row_idx, 0);

        for column_idx in (0..MAX_COLUMN).into_iter() {
            let point = (row_idx, column_idx);
            if rocks.contains(&point) {
                next_available = (row_idx, column_idx + 1);
            }

            if boulders.contains(&point) {
                new_boulders.insert(next_available);
                next_available = (row_idx, next_available.1 + 1);
            }
        }
    }

    new_boulders
}

fn get_load(boulders: &HashSet<(usize, usize)>) -> usize {
    boulders.into_iter().map(|(row_idx, _)| MAX_ROW - row_idx).sum()
}

fn get_key(boulders: &HashSet<(usize, usize)>) -> String {
    let mut keys: Vec<String> = boulders.iter().map(|(r,c)| (((*r) * MAX_COLUMN) + *c).to_string() ).collect();
    keys.sort();
    keys.join(",")
}

fn main() {
    let (rocks, boulders) = parse_input();

    let mut cache = HashMap::new();
    let mut load_cache = Vec::new();
    let mut after_cycle = boulders;
    let mut finish = &after_cycle;
    let mut last_cycle = 0;
    for i in (0..1_000_000_000).into_iter() {
        let current_key = get_key(finish);

        last_cycle = i + 1;         

        if cache.contains_key(&current_key) {
            finish = cache.get(&current_key).unwrap();
            let load  = get_load(&finish);
            if load_cache.contains(&load) {
                break;
            } else {
                load_cache.push(load);
            }
        } else {
            after_cycle = tilt_north(&rocks, finish);
            after_cycle = tilt_west(&rocks, &after_cycle);
            after_cycle = tilt_south(&rocks, &after_cycle);
            after_cycle = tilt_east(&rocks, &after_cycle);

            cache.insert(current_key.clone(), after_cycle);

            finish = cache.get(&current_key).unwrap();
        }       
    }

    let index =(1_000_000_000 - last_cycle) % load_cache.len();
    println!("{}",  load_cache[index]); // 102055
}

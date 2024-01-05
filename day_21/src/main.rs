use std::{fs::read_to_string, collections::{HashSet, HashMap}};
use priority_queue::PriorityQueue;

fn parse_input() -> (HashSet<(i64, i64)>, (i64, i64)) {
    let mut rocks = HashSet::new();
    let mut start = Vec::new();

    for (r_idx, row) in read_to_string("./data.txt").unwrap().lines().enumerate() {
        let row_idx: i64 = r_idx.try_into().unwrap();
        for (c_idx, space) in row.chars().enumerate() {
            let column_idx: i64 = c_idx.try_into().unwrap();
            match space {
                'S' => { start.push((row_idx, column_idx)); },
                '#' => { rocks.insert((row_idx, column_idx)); }
                c => { assert!(c == '.'); }
            }
        }
    }

    (rocks, start[0])
}

const SIZE: i64 = 131;
const N: i64 = 4;

fn is_rock(point: &(i64, i64), rocks: &HashSet<(i64, i64)>) -> bool {
    let (r,c) = point;
    rocks.contains(&((SIZE + (*r % SIZE)) % SIZE, (SIZE + (*c % SIZE)) % SIZE))
}

fn count_square(row_idx: i64, column_idx: i64, distances: &HashMap<(i64, i64), i64>) -> i64 {
    let lower_r = row_idx*SIZE;
    let upper_r = (row_idx+1)*SIZE;
    let lower_c = column_idx*SIZE;
    let upper_c = (column_idx+1)* SIZE;

    let mut odd_count = 0;
    for r_idx in lower_r..upper_r {
        for c_idx in lower_c..upper_c {
            if let Some(dist) = distances.get(&(r_idx, c_idx)) {
                if *dist <= (N*131 + 65) && *dist % 2 != 0 {
                    odd_count += 1;
                }
            }
        }
    }

    odd_count
}

fn dijkstra(start: &(i64, i64), rocks: &HashSet<(i64, i64)>) -> HashMap<(i64, i64), i64> {
    let lower_limit = -1 * N * SIZE;
    let upper_limit = (N+1) * SIZE;
    let mut to_do = PriorityQueue::new();
    to_do.push(start.to_owned(), 0);
    let mut distance = HashMap::from([(start.to_owned(), 0)]);


    while let Some(((r,c), priority)) = to_do.pop() {
        let curr_distance = -1* priority;
        let neigbours: Vec<(i64, i64)> = Vec::from([(r+1,c), (r-1,c), (r,c+1), (r,c-1)]).iter()
            .filter(|(nr, nc)| *nr>=lower_limit && *nr < upper_limit && *nc >=lower_limit && *nc < upper_limit)
            .filter(|n| !is_rock(n, rocks))
            .map(|n| n.to_owned())
            .collect();

        for neighbour in neigbours {
            if let Some(ndist) =distance.get_mut(&neighbour) {
                if curr_distance +1 < *ndist {
                    *ndist = curr_distance +1;
                    to_do.change_priority(&neighbour, -1 * (curr_distance + 1));
                }

            } else {
                distance.insert(neighbour.clone(), curr_distance + 1);
                to_do.push(neighbour, priority - 1);
            }
        }

    }

    distance
}

fn main() {
    let (rocks, start) = parse_input();

    let distances = dijkstra(&start, &rocks);

    let with_64_steps = distances.iter().filter(|(_, d)| **d % 2 ==0 && **d <= 64 ).collect::<Vec<_>>().len();
    println!("{}", with_64_steps); //3649

    let full_odd_count = count_square(0, 0, &distances);
    let full_even_count = count_square(1, 0, &distances);

    let top_mid = count_square(-4, 0, &distances);
    let bottom_mid = count_square(4, 0, &distances);
    let left_mid = count_square(0, -4, &distances);
    let right_mid = count_square(0, 4, &distances);

    let top_left_big = count_square(-3, -1, &distances);
    let top_right_big = count_square(-3, 1, &distances);
    let top_left_small = count_square(-4, -1, &distances);
    let top_right_small = count_square(-4, 1, &distances);

    let bottom_left_big = count_square(3, -1, &distances);
    let bottom_right_big = count_square(3, 1, &distances);
    let bottom_left_small = count_square(4, -1, &distances);
    let bottom_right_small = count_square(4, 1, &distances);

    let count_even_corners = top_left_small+top_right_small+bottom_left_small+bottom_right_small;
    let count_odd_corners = top_left_big+top_right_big+bottom_left_big+bottom_right_big;
    let count_mid = top_mid+bottom_mid+left_mid+right_mid;

    let n = 202300;  // (26501365 - 65) / 131

    let total = ((n-1)*(n-1)) * (full_odd_count) + (n*n) * (full_even_count) + (n-1) * count_odd_corners + n * count_even_corners + count_mid;
    println!("{}", total); // 612941134797232
}

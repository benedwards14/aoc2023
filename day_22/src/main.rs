use std::{fs::read_to_string, collections::{HashMap, HashSet, VecDeque}};

use priority_queue::PriorityQueue;

fn parse_input() -> HashMap<i32, Brick> {
    let mut bricks = HashMap::new();

    for (row_idx, row) in read_to_string("./data.txt").unwrap().lines().enumerate() {
        let brick_id = row_idx.try_into().unwrap();
        let (start_coord, end_coord) = row.split_once('~').unwrap();
        bricks.insert(brick_id, Brick { brick_id,  start: Coord::parse(start_coord), end: Coord::parse(end_coord) });
    }

    bricks
}

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
    z: i32
}

impl Coord {
    fn parse(line: &str) -> Self {
        let coords: Vec<&str> = line.split(',').collect();

        Self {
            x: coords[0].parse().unwrap(),
            y: coords[1].parse().unwrap(),
            z: coords[2].parse().unwrap()
        }
    }
    
}

#[derive(Debug)]
struct Brick {
    brick_id: i32,
    start: Coord,
    end: Coord
}

impl Brick {
    fn get_height(&self) -> i32 {
        self.end.z - self.start.z + 1
    }

    fn get_start_xy(&self) -> (i32, i32) {
        (self.start.x, self.start.y)
    }
}

fn get_supported_by(bricks: &HashMap<i32, Brick>) -> HashMap<i32, Vec<i32>> {
    let mut supporting_bricks: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut current_height: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    let mut sorted_bricks: Vec<&Brick> =  bricks.values().collect();
    sorted_bricks.sort_by_key(|b| b.start.z);

    for brick in sorted_bricks {
        let mut cross_section = Vec::new();
        let mut heights = HashSet::new();

        for x in brick.start.x..=brick.end.x {
            for y in brick.start.y..=brick.end.y {
                cross_section.push((x, y));
                if let Some((curr_top, curr_height)) = current_height.get(&(x,y)) {
                    heights.insert((*curr_top, *curr_height));
                }
            }
        }

        let max_height = heights.iter().map(|(_,h)| *h).max().unwrap_or(0);
        let supporting: Vec<i32> = heights.iter()
            .filter(|(_, h)| *h == max_height)
            .map(|(b, h)| *b)
            .collect();

        supporting_bricks.insert(brick.brick_id, supporting);
        let new_height = max_height + brick.get_height();
        cross_section.iter().for_each(|coord| { current_height.insert(*coord, (brick.brick_id, new_height)); });
    }

    supporting_bricks
}

fn get_supported(supporting_bricks: &HashMap<i32, Vec<i32>>) -> HashMap<i32, HashSet<i32>> {
    let mut supported_by: HashMap<i32, HashSet<i32>> = HashMap::new();

    for (brick, supports) in supporting_bricks {
        for brick_id in supports {
            if let Some(curr) = supported_by.get_mut(brick_id) {
                (*curr).insert(*brick);
            } else {
                supported_by.insert(*brick_id, HashSet::from([*brick]));
            }
        }
    }

    supported_by
}

fn count_falling(disintegrated_brick: i32, supporting: &HashMap<i32, HashSet<i32>>, supported_by: &HashMap<i32, Vec<i32>>, bricks: &HashMap<i32, Brick>) -> usize {
    let mut fallen = HashSet::from([disintegrated_brick]);
    let mut to_do = PriorityQueue::new();
    to_do.push(disintegrated_brick, -1 * bricks.get(&disintegrated_brick).unwrap().start.z);

    while let Some((brick_id, _)) = to_do.pop() {
        let default = HashSet::new();
        let supports = supporting.get(&brick_id).unwrap_or(&default);

        for s in supports {
            let s_by = supported_by.get(s).unwrap();
            let will_fall = s_by.iter().all(|sb| fallen.contains(sb));

            if will_fall {
                fallen.insert(*s);
                to_do.push(*s, -1 * bricks.get(&disintegrated_brick).unwrap().start.z);
            }
        }
    }

    fallen.len() - 1
}

fn main() {
    let bricks = parse_input();
    let supported_by = get_supported_by(&bricks);
    let supporting= get_supported(&supported_by);

    // println!("{:?} {:?}", supported_by .get(&0).unwrap(), supporting.get(&0).unwrap());

    let unstable_bricks: HashSet<i32> = supported_by.values()
        .filter(|s| s.len() == 1)
        .map(|s| s[0])
        .collect();

    println!("{:?}", bricks.len() - unstable_bricks.len()); // 446

    let total: usize = unstable_bricks.iter().map(|ub| count_falling(*ub, &supporting, &supported_by, &bricks)).sum();
    println!("{:?}", total);
}

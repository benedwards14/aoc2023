use std::{fs::read_to_string, collections::{HashMap, HashSet}};
use priority_queue::PriorityQueue;

fn parse_input() -> Map  {
    let mut blocks = HashMap::new();
    for (row_idx, row) in read_to_string("./data.txt").unwrap().lines().enumerate() {
        for (column_idx, space) in row.chars().enumerate() {
            let value = space.to_digit(10).unwrap();
            blocks.insert((row_idx.try_into().unwrap(), column_idx.try_into().unwrap()), value.try_into().unwrap());
        }
    }

    Map { blocks }
}

const SIZE: i32 = 141;
const MAX_STEP: i32 = 10;
const MIN_STEP: i32 = 4;

struct Map {
    blocks: HashMap<(i32, i32), i32>
}

#[derive(PartialEq, Hash, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq, Hash, Eq, Clone)]
struct Node {
    pos: (i32, i32),
    direction: Direction,
    current_steps: i32
}

impl Direction {
    fn step(&self, current: &(i32, i32)) -> Option<(i32, i32)> {
        let (row, col) = *current;
        let (next_row, next_col) = match self {
            Self::Up => (row - 1, col),
            Self::Down => (row + 1, col),
            Self::Left => (row, col - 1),
            Self::Right => (row, col + 1),
        };
        if next_row < 0 || next_row >= SIZE || next_col < 0 || next_col >= SIZE {
            return None;
        }

        Some((next_row, next_col))
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }
}

impl Node {
    fn get_neighbours(&self) -> Vec<Self> {
        let mut neigbours = Vec::new();
        if self.current_steps < MAX_STEP {
            if let Some(new) = self.direction.step(&self.pos) {
                neigbours.push(Node { pos: new, direction: self.direction.clone(), current_steps: self.current_steps + 1 })
            }
        }
        if self.current_steps >= MIN_STEP {
            let left_direction = self.direction.turn_left();
            if let Some(new) = left_direction.step(&self.pos) {
                neigbours.push(Node { pos: new, direction: left_direction, current_steps: 1 })
            }
            let right_direction = self.direction.turn_right();
            if let Some(new) = right_direction.step(&self.pos) {
                neigbours.push(Node { pos: new, direction: right_direction, current_steps: 1 })
            }
        }

        neigbours
    }

    fn is_end(&self) -> bool {
        let (r,c) = self.pos;
        r == (SIZE-1) && c == (SIZE-1)
    }
}

impl Map {
    fn walk(&self) -> i32 {
        let mut visited: HashSet<Node> = HashSet::from([
            Node { pos: (0,0), direction: Direction::Right, current_steps: 1 }, 
            Node { pos: (0,0), direction: Direction::Down, current_steps: 1 }
        ]);

        let mut to_visit =  PriorityQueue::new();
        to_visit.push(
            Node { pos: (1,0), direction: Direction::Down, current_steps: 2 }, 
            -1 * self.blocks.get(&(1,0)).unwrap()
        );
        to_visit.push(
            Node { pos: (0,1), direction: Direction::Right, current_steps: 2 },
            -1 * self.blocks.get(&(0,1)).unwrap()
        );

        while let Some((curr, curr_total)) = to_visit.pop() {
            for neighbour in curr.get_neighbours() {
                if visited.contains(&neighbour) { continue; };

                let n_local = *self.blocks.get(&neighbour.pos).unwrap();
                let new_n_total = curr_total - n_local;

                if neighbour.is_end() {
                    return -1 * new_n_total;
                } else if let Some((_, old_n_total)) = to_visit.get(&neighbour) {
                    if new_n_total > *old_n_total {
                        to_visit.change_priority(&neighbour, new_n_total);
                    }
                } else {
                    to_visit.push(neighbour, new_n_total);
                }
            }

            visited.insert(curr);
        }

        unreachable!()
    }
}

fn main() {
    let map = parse_input();
    println!("{}", map.walk());
}


use std::{fs::read_to_string, collections::HashMap};

struct Game {
    id: u32,
    actual: Vec<i32>,
    guessed: Vec<i32>
}

fn parse_numbers(line: &str) -> Vec<i32> {
    let mut numbers: Vec<i32> =  Vec::new();

    for n in line.split_whitespace() {
        match n {
            "" => {},
            _ => { numbers.push(n.parse().unwrap()) }
        }
    }

    numbers
}

fn parse_input() -> Vec<Game> {
    let mut games = Vec::new();

    for (row_idx, line) in read_to_string("./data.txt").unwrap().lines().enumerate() {
        let (_, game) = line.split_once(":").unwrap();
        let (actual, guessed) = game.split_once("|").unwrap();

        games.push(Game {
            id: (row_idx + 1).try_into().unwrap(),
            actual: parse_numbers(actual.trim()),
            guessed: parse_numbers(guessed.trim())
        })
    }

    games
}

impl Game {
    fn get_num_correct_guesses(&self) -> u32 {
        self.guessed.iter().filter(|n| self.actual.contains(n)).collect::<Vec<_>>().len().try_into().unwrap()
    }
}

fn main() {
    let input = parse_input();

    let total: i32 = input.iter().map(
        |g| {
            let correct: u32 = g.get_num_correct_guesses();
            if correct > 0 {
                2_i32.pow(correct - 1)
            } else { 0 }
        }
    ).sum();

    println!("{}", total);

    let mut copies = HashMap::new();
    input.iter().for_each(|g| { copies.insert(g.id, 1_u32); });

    input.iter().for_each(|g| {
        let number_of_copies = *copies.get(&g.id).unwrap();
        let correct = g.get_num_correct_guesses();
        for idx in 1..=correct {
            *copies.get_mut(&(g.id + idx)).unwrap() += number_of_copies;
        }
    });

    let total2: u32 = copies.values().sum();
    println!("{}", total2);

    
}

use std::fs::read_to_string;
use std::collections::HashMap;

struct Round {
    red: i32,
    green: i32,
    blue: i32
}

impl Round {
    fn parse(line: &str) -> Self {
        let mut values = HashMap::new();

        line.split(",").map(
            |c| c.trim().split_once(" ").unwrap()
        ).for_each(
            |(count, colour)|  { values.insert(colour.trim(), count.trim().parse::<i32>().unwrap()); }
        );

        Round {
            red: *values.get("red").unwrap_or(&0),
            green: *values.get("green").unwrap_or(&0),
            blue: *values.get("blue").unwrap_or(&0)
        }
    }

    fn possible_with(&self, red: i32, green: i32, blue: i32) -> bool {
        self.red <= red &&
        self.green <= green &&
        self.blue <= blue
    }
}

struct Game {
    id: i32,
    rounds: Vec<Round>
}

impl Game {
    fn parse(line: &str) -> Self {
        let (game, rounds) = line.split_once(":").unwrap();
        let id = game.trim()[5..].parse::<i32>().unwrap();
        let parsed_rounds = rounds.trim().split(";").map(|r| Round::parse(r.trim())).collect();

        Game { 
            id,
            rounds: parsed_rounds
        }
    }

    fn possible_with(&self, red: i32, green: i32, blue: i32) -> bool {
        self.rounds.iter().all(
            |r| r.possible_with(red, green, blue)
        )
    }

    fn get_power(&self) -> i32 {
        let red = self.rounds.iter().map(|r| r.red).max().unwrap();
        let green = self.rounds.iter().map(|r| r.green).max().unwrap();
        let blue = self.rounds.iter().map(|r| r.blue).max().unwrap();

        red * green * blue
    }
}

fn parse_input() -> Vec<Game> {
    let mut result: Vec<Game> = Vec::new();

    for line in read_to_string("./data.txt").unwrap().lines() {
        result.push(Game::parse(line))
    }

    result
}

fn main() {
    let input = parse_input();

    let sum: i32 = input.iter()
        .filter(|g| g.possible_with(12, 13, 14))
        .map(|g| g.id)
        .sum();

    println!("{}", sum);

    let power: i32 = input.iter()
        .map(|g| g.get_power())
        .sum();

    println!("{}", power);
}

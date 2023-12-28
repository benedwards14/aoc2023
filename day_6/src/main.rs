struct Race {
    time: i64,
    record: i64
}

fn parse_input() -> Vec<Race> {
    Vec::from([
        Race { time: 47, record: 400 },
        Race { time: 98, record: 1213 },
        Race { time: 66, record: 1011 },
        Race { time: 98, record: 1540 },
    ])
}

impl Race {
    fn get_wins(&self) -> Vec<i64> {
        let mut wins = Vec::new();
        for charge_time in 0..=self.time {
            let distance = charge_time * (self.time - charge_time);
            if distance > self.record {
                wins.push(charge_time);
            }
        }

        wins
    }
}

fn get_all_wins(races: Vec<Race>) -> Vec<i64> {
    races.iter()
    .map(|r| r.get_wins())
    .map(|w| w.len().try_into().unwrap())
    .collect()
}

fn main() {
    let input = parse_input();

    let number_of_wins = get_all_wins(input);

    let mut sum = 1;
    for nw in number_of_wins {
        sum *= nw;
    }

    println!("{}", sum);

    let sum2 = get_all_wins(Vec::from([Race {time: 47986698, record: 400121310111540}]))[0];
    println!("{}", sum2);
}

use std::fs::read_to_string;
use std::collections::HashMap;

static NUMBERS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
];

fn parse_input() -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string("./data.txt").unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn get_value_from_string(word: &str) -> u32 {
    let mut matches: Vec<(usize, &str)> = Vec::new();

    for possible_num in NUMBERS {
        let mut mi = word.match_indices(possible_num);

        if let Some(x) =  mi.next() {
            matches.push(x);

            if let Some(y) =  mi.last() {
                matches.push(y);
            }
        }
        
    }

    matches.sort_by_key(|k| k.0);

    let first = matches[0].1;
    let last = matches.last().unwrap_or(&matches[0]).1;

    let first_digit = to_digit(first);
    let last_digit = to_digit(last);

    first_digit * 10 + last_digit
}

fn to_digit(word: &str) -> u32 {
    let map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);
    
    word.chars().collect::<Vec<_>>()[0].to_digit(10).unwrap_or_else(
        || *map.get(word).unwrap()
    )
}

fn main() {
    let words: Vec<String> = parse_input();
    let mut sum = 0;
    for word in &words {
        sum += get_value_from_string(word)
    }

    println!("{}", sum)
}

// 55971

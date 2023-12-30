use std::fs::read_to_string;

fn parse_input() -> Vec<Pattern> {
    let mut patterns = Vec::new();

    let mut current_pattern = Vec::new();
    let input = read_to_string("./data.txt").unwrap();
    for row in input.lines() {
        match row {
            "" => { 
                patterns.push(Pattern::parse(current_pattern));
                current_pattern = Vec::new();
            }
            a => { current_pattern.push(a); }
        }
    }

    assert!(!current_pattern.is_empty());
    patterns.push(Pattern::parse(current_pattern));

    patterns
}

struct Pattern {
    rows: Vec<i32>,
    columns: Vec<i32>
}

impl Pattern {
    fn parse(strings: Vec<&str>) -> Self {
        let digits: Vec<Vec<&str>> = strings
            .iter()
            .map(
                |s| {
                    s.chars().map(
                        |c| {
                            match c {
                                '.' => "0",
                                '#' => "1",
                                _ => panic!("Oh No!")
                            }
                        }
                    ).collect()
                }
            ).collect();
        
        let rows: Vec<i32> = digits.iter().map(|r| i32::from_str_radix(&r.join(""), 2).unwrap()).collect();
        let columns: Vec<i32> = (0..digits[0].len()).into_iter().map(
            |idx| digits.iter().map(|d| d[idx]).collect::<Vec<&str>>()
        ).map(|r| i32::from_str_radix(&r.join(""), 2).unwrap()).collect();

        Self { rows, columns }
    }
    
}


fn find_reflection(row: &Vec<i32>, required_smudges: i32) -> Option<usize> {
    let identical: Vec<usize> = (0..(row.len() -1)).into_iter()
        .map(|idx| (idx, row[idx]))
        .filter(|(idx, value)| *value == row[idx + 1] || is_single_smudge(*value, row[idx + 1]))
        .map(|(idx, _)| idx)
        .collect();

    for idx in identical {
        let mut lower_idx = idx;
        let mut upper_idx = idx+1;
        let mut number_of_smudges = 0;
        loop {
            let lower = row[lower_idx];
            let upper = row[upper_idx];

            if lower == upper {
            } else if is_single_smudge(lower, upper) {
                number_of_smudges += 1;
            } else {
                break;
            }
    
            if lower_idx == 0 || upper_idx == (row.len() -1) {
                if number_of_smudges == required_smudges {
                    return Some(idx + 1);
                } 
                break;
            }
            lower_idx -= 1;
            upper_idx += 1;
        }     
    }

    None
}

fn is_single_smudge(first: i32, second: i32) -> bool {
    let difference = first ^ second;

    (difference & (difference - 1)) == 0
}

fn find_axis_of_reflection(pattern: &Pattern, required_smudges: i32) -> usize {
    match (find_reflection(&pattern.rows, required_smudges), find_reflection(&pattern.columns, required_smudges)) {
        (None, Some(column)) => column,
        (Some(row), None) => row * 100,
        _ => panic!("Oh no!")
    }
}


fn main() {
    let input = parse_input();

    let sum1: usize = input.iter().map(|p| find_axis_of_reflection(p, 0)).sum();
    println!("{}", sum1); //30518

    let sum2: usize = input.iter().map(|p| find_axis_of_reflection(p, 1)).sum();
    println!("{}", sum2); //36735
}

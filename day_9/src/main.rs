use std::fs::read_to_string;

fn parse_input() -> Vec<Vec<i32>> {
    let mut sequences = Vec::new();
    let input_str = read_to_string("./data.txt").unwrap();
    let mut lines = input_str.lines();

    for line in lines {
        let numbers:Vec<i32> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();
        sequences.push(numbers);
    }

    sequences
}

struct SequencElement {
    differences: Vec<i32>
}

struct Sequence {
    first: SequencElement,
    last: SequencElement,
}

impl SequencElement {
    fn next(&self) -> Self {
        let mut new_differences = Vec::new();
        let mut total = 0;

        for old_difference in &self.differences {
            total += old_difference;
            new_differences.push(total);
        }

        SequencElement { differences: new_differences}
    }

    fn previous(&self) -> Self {
        let mut new_differences = Vec::new();
        let mut total = 0;

        for old_difference in &self.differences {
            total = old_difference - total;
            new_differences.push(total);
        }

        SequencElement { differences: new_differences}
    }

    fn value(&self) -> i32 {
        *self.differences.last().unwrap()
    }
    
}

fn create_sequence(sequence: &Vec<i32>) -> Sequence {
    if sequence.iter().all(|e| *e == 0) {
        return Sequence { first: SequencElement { differences : Vec::new() } , last: SequencElement { differences : Vec::new() } }
    }

    let mut differences = Vec::new();
    let len = sequence.len();
    for idx in 1..len {
        differences.push(sequence[idx] - sequence[idx-1]);
    }

    let mut parsed_sequence = create_sequence(&differences);

    let last_element = *sequence.last().unwrap();
    let first_element = sequence[0];
    parsed_sequence.last.differences.push(last_element);
    parsed_sequence.first.differences.push(first_element);

    parsed_sequence
}

fn main() {
    let input = parse_input();

    let sequences: Vec<Sequence> = input
        .iter()
        .map(|s| create_sequence(s))
        .collect(); 
    let total: i32 = sequences
        .iter()
        .map(|s| &s.last)
        .map(|e| e.next())
        .map(|e| e.value())
        .sum();

    println!("{}", total); //1992273652

    let total2: i32 = sequences
        .iter()
        .map(|s| &s.first)
        .map(|e| e.previous())
        .map(|e| e.value())
        .sum();

    println!("{}", total2); 

}

use std::{fs::read_to_string, collections::{HashSet}};

const MAX_ROW: i64 = 140;
const MAX_COLUMN: i64 = 140;

fn parse_input() -> Vec<(i64, i64)> {
    let mut galaxies = Vec::new();

    for (idx, row) in read_to_string("./data.txt").unwrap().lines().enumerate() {
        let row_idx: i64  = idx.try_into().unwrap();
        for (idx, column) in row.chars().enumerate() {
            let column_idx: i64 = idx.try_into().unwrap();
            match column {
                '.' => {continue;},
                '#' => {galaxies.push((row_idx, column_idx));},
                _ => panic!("Oh No!")
                
            }
        }
    }

    galaxies
}

fn adjust_for_empty(galaxies: &Vec<(i64, i64)>, scale_factor: i64) -> Vec<(i64, i64)> {
    let mut adjusted_galaxies = galaxies.to_vec();
    let rows = adjusted_galaxies.iter().map(|g| g.0).collect::<HashSet<i64>>();
    let columns = adjusted_galaxies.iter().map(|g| g.1).collect::<HashSet<i64>>();

    let mut empty_rows: Vec<i64> = (0..MAX_ROW).into_iter().filter(|r| !rows.contains(r)).collect();
    let mut empty_columns: Vec<i64> = (0..MAX_COLUMN).into_iter().filter(|c| !columns.contains(c)).collect();

    empty_rows.sort();
    empty_columns.sort();

    for empty_row in empty_rows.iter().rev() {
        adjusted_galaxies = adjusted_galaxies.iter().map(|(row, column)| if *row > *empty_row { (row + scale_factor -1, *column) } else { (*row, *column) }).collect();
    }

    for empty_column in empty_columns.iter().rev() {
        adjusted_galaxies = adjusted_galaxies.iter().map(|(row, column)| if *column > *empty_column { (*row, column + scale_factor -1) } else { (*row, *column) }).collect();
    }

    adjusted_galaxies
}

fn find_distances(galaxies: &Vec<(i64, i64)>) -> Vec<i64> {
    let mut distances = Vec::new();

    for (row_1, column_1) in galaxies {
        for (row_2, column_2) in galaxies {
            let distance = (row_2 - row_1).abs() + (column_2 - column_1).abs();

            if row_2 > row_1 || (row_2 == row_1 && column_2 > column_1) {
                distances.push(distance);
            }
        }
    }

    distances
}

fn main() {
    let galaxies = parse_input();

    let adjusted_galaxies1 = adjust_for_empty(&galaxies, 2);
    let distances1 = find_distances(&adjusted_galaxies1);
    let total1: i64 = distances1.iter().sum::<i64>();
    println!("{}" , total1);

    let adjusted_galaxies2 = adjust_for_empty(&galaxies, 1000000);
    let distances2 = find_distances(&adjusted_galaxies2);
    let total2: i64 = distances2.iter().sum::<i64>();
    println!("{}" , total2);
}

use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn read_numbers_file_into_hashset(file:File) -> HashSet<i32> {
    return BufReader::new(file)
        .lines()
        .map(|nrs| nrs.unwrap().parse::<i32>().unwrap())
        .collect();
}

fn part_1(to_test_numbers: HashSet<i32>) -> i32 {
    return to_test_numbers
        .iter()
        .map(|a| (a, 2020 - *a))
        .filter(|(_, b)| to_test_numbers.contains(b))
        .map(|(a, b)| a * b)
        .next()
        .unwrap();
}

fn part_2(to_test_numbers: HashSet<i32>) -> i32 {
    return to_test_numbers
        .iter()
        .cartesian_product(to_test_numbers.iter())
        .map(|(a, b)| (a, b, 2020 - *a - *b))
        .filter(|(_, _, c)| to_test_numbers.contains(c))
        .map(|(a, b, c)| a * b * c)
        .next()
        .unwrap();
}

fn main() {
    let tt1 = Instant::now();

    let file = File::open("input.txt").unwrap();
    let to_test_numbers = read_numbers_file_into_hashset(file);

    let t1 = Instant::now();
    println!(
        "Day 1 - 1: {} solved in {}µs",
        part_1(to_test_numbers.clone()),
        t1.elapsed().as_micros()
    );

    let t2 = Instant::now();
    println!(
        "Day 1 - 2: {} solved in {}µs",
        part_2(to_test_numbers.clone()),
        t2.elapsed().as_micros()
    );

    println!("Complete code execution took {}µs", tt1.elapsed().as_micros())
}
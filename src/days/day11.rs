use std::{collections::HashMap, fs};

fn part1(input: &str) -> i64 {
    let mut stone_counts: HashMap<i64, i64> = input
        .trim()
        .split_whitespace()
        .map(|s| (s.parse::<i64>().unwrap(), 1))
        .collect();

    for _ in 0..25 {
        stone_counts = transform_stones(stone_counts);
    }

    stone_counts.values().sum()
}

fn transform_stones(stones: HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut new_stones: HashMap<i64, i64> = HashMap::new();
    
    for (stone, count) in stones {
        if stone == 0 {
            *new_stones.entry(1).or_insert(0) += count;
        } else {
            let digits = stone.to_string();

            if digits.len() % 2 == 0 {
                let mid = digits.len() / 2;
                let left = digits[..mid].parse::<i64>().unwrap();
                let right = digits[mid..].parse::<i64>().unwrap();
                
                *new_stones.entry(left).or_insert(0) += count;
                *new_stones.entry(right).or_insert(0) += count;
            } else {
                *new_stones.entry(stone * 2024).or_insert(0) += count;
            }
        }
    }
    new_stones
}

fn part2(input: &str) -> i64 {
     let mut stone_counts: HashMap<i64, i64> = input
        .trim()
        .split_whitespace()
        .map(|s| (s.parse::<i64>().unwrap(), 1))
        .collect();

    for _ in 0..75 {
        stone_counts = transform_stones(stone_counts);
    }
    
    stone_counts.values().sum()
}

pub fn run() {
    let start = std::time::Instant::now();

    let input = fs::read_to_string("./src/input/day11.txt").expect("Failed to read input file");

    println!("Day 11 - Part 1: {}", part1(&input));
    let part1_duration = start.elapsed();
    println!("Day 11 - Part 1 duration: {:?}", part1_duration);

    println!("Day 11 - Part 2: {}", part2(&input));
    let part2_duration = start.elapsed() - part1_duration;
    println!("Day 11 - Part 2 duration: {:?}", part2_duration);

    println!("Day 11 - Total duration: {:?}", start.elapsed());
}
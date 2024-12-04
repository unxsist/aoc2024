use std::io::BufRead;
use std::collections::HashMap;

fn parse_input() -> Result<(Vec<i32>, Vec<i32>), std::io::Error> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    let file = std::fs::File::open("./src/input/day01.txt")?;
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        let left = parts.next().unwrap().parse().unwrap();
        let right = parts.next().unwrap().parse().unwrap();
        left_list.push(left);
        right_list.push(right);
    }

    Ok((left_list, right_list))
}

pub fn part1(mut left_list: Vec<i32>, mut right_list: Vec<i32>) -> i32 {
    left_list.sort();
    right_list.sort();

    left_list.iter()
        .zip(right_list.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

pub fn part2(left_list: Vec<i32>, right_list: Vec<i32>) -> i32 {
    let mut right_count = HashMap::new();
    for r in right_list {
        *right_count.entry(r).or_insert(0) += 1;
    }

    left_list.into_iter()
        .map(|l| l * right_count.get(&l).unwrap_or(&0))
        .sum()
}

pub fn run() {
    let lists = parse_input().unwrap();

    let start = std::time::Instant::now();

    println!("Day 1 - Part 1: {}", part1(lists.0.clone(), lists.1.clone()));
    let part1_duration = start.elapsed();
    println!("Day 1 - Part 1 duration: {:?}", part1_duration);

    println!("Day 1 - Part 2: {}", part2(lists.0.clone(), lists.1.clone()));
    let part2_duration = start.elapsed() - part1_duration;
    println!("Day 1 - Part 2 duration: {:?}", part2_duration);

    println!("Day 1 - Total duration: {:?}", start.elapsed());
}
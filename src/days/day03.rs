use std::fs;
use regex::Regex;

pub fn part1() -> i32 {
    let input = fs::read_to_string("./src/input/day03.txt")
        .expect("Failed to read the input file");
    
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    
    let mut total_sum = 0;
    for cap in re.captures_iter(&input) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        total_sum += x * y;
    }

    total_sum
}

pub fn part2() -> i32 {
    let input = fs::read_to_string("./src/input/day03.txt")
    .expect("Failed to read the input file");

    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    
    let mut total_sum = 0;
    let mut enabled = true;

    for cap in re.captures_iter(&input) {
        if let Some(_) = cap.get(0) {
            if cap.get(1).is_some() && cap.get(2).is_some() {
                if enabled {
                    let x: i32 = cap[1].parse().unwrap();
                    let y: i32 = cap[2].parse().unwrap();
                    total_sum += x * y;
                }
            } else if cap.get(0).unwrap().as_str() == "do()" {
                enabled = true;
            } else if cap.get(0).unwrap().as_str() == "don't()" {
                enabled = false;
            }
        }
    }

    total_sum
}

pub fn run() {
    let start = std::time::Instant::now();

    println!("Day 3 - Part 1: {}", part1());
    let part1_duration = start.elapsed();
    println!("Day 3 - Part 1 duration: {:?}", part1_duration);

    println!("Day 3 - Part 2: {}", part2());
    let part2_duration = start.elapsed() - part1_duration;
    println!("Day 3 - Part 2 duration: {:?}", part2_duration);

    println!("Day 3 - Total duration: {:?}", start.elapsed());
}
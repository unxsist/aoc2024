use std::fs::read_to_string;

fn parse_input() -> Result<Vec<Vec<i32>>, std::io::Error> {
    let input = read_to_string("./src/input/day02.txt")?;
    Ok(input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect()
        })
        .collect())
}

fn is_safe(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return false;
    }

    let differences: Vec<i32> = report.windows(2).map(|pair| pair[1] - pair[0]).collect();

    if differences.iter().any(|&diff| diff.abs() < 1 || diff.abs() > 3) {
        return false;
    }

    let all_increasing = differences.iter().all(|&diff| diff > 0);
    let all_decreasing = differences.iter().all(|&diff| diff < 0);

    all_increasing || all_decreasing
}

fn is_safe_with_dampener(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);
        if is_safe(&modified_report) {
            return true; 
        }
    }

    false 
}

fn part1() -> Result<usize, std::io::Error> {
    let reports = parse_input()?;
    let safe_count = reports.iter().filter(|report| is_safe(report)).count();
    Ok(safe_count)
}

fn part2() -> Result<usize, std::io::Error> {
    let reports = parse_input()?;
    let safe_count = reports.iter().filter(|report| is_safe_with_dampener(report)).count();
    Ok(safe_count)
}

pub fn run() {
    let start = std::time::Instant::now();

    match part1() {
        Ok(safe_count) => println!("Day 2 - Part 1: {}", safe_count),
        Err(err) => eprintln!("Error reading input: {}", err),
    }

    let part1_duration = start.elapsed();
    println!("Day 2 - Part 1 duration: {:?}", part1_duration);

    match part2() {
        Ok(safe_count) => println!("Day 2 - Part 2: {}", safe_count),
        Err(err) => eprintln!("Error reading input: {}", err),
    }

    let part2_duration = start.elapsed() - part1_duration;
    println!("Day 2 - Part 2 duration: {:?}", part2_duration);

    let total_duration = start.elapsed();
    println!("Day 2 - Total duration: {:?}", total_duration);
}
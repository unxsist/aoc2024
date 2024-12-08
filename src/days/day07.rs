use std::fs;

fn part1(equations: &Vec<(i64, Vec<i64>)>) -> i64 {
    equations.iter()
        .filter_map(|(target, numbers)| {
            if can_form_target(*target, numbers) {
                Some(*target)
            } else {
                None
            }
        })
        .sum()
}

fn can_form_target(target: i64, numbers: &Vec<i64>) -> bool {
    let operator_slots = numbers.len() - 1;
    let total_combinations = 1 << operator_slots;

    for combination in 0..total_combinations {
        let mut result = numbers[0];
        for (i, &num) in numbers[1..].iter().enumerate() {
            if (combination & (1 << i)) != 0 {
                result += num;
            } else {
                result *= num;
            }
        }
        if result == target {
            return true;
        }
    }
    false
}

fn part2(equations: &Vec<(i64, Vec<i64>)>) -> i64 {
equations
   .iter()
   .filter_map(|(target, numbers)| {
       if can_form_target_with_concat(*target, numbers) {
           Some(*target)
       } else {
           None
       }
   })
   .sum()
}

fn can_form_target_with_concat(target: i64, numbers: &Vec<i64>) -> bool {
    let operator_slots = numbers.len() - 1;
    let total_combinations = 3_i64.pow(operator_slots as u32);

    for combination in 0..total_combinations {
        let mut result = numbers[0];
        let mut combination_index = combination;

        for (_i, &num) in numbers[1..].iter().enumerate() {
            let operator_type = combination_index % 3;
            combination_index /= 3;

            match operator_type {
                0 => result += num,
                1 => result *= num,
                2 => {
                    let concatenated = format!("{}{}", result, num)
                        .parse::<i64>()
                        .unwrap();
                    result = concatenated;
                }
                _ => unreachable!(),
            }
        }
        if result == target {
            return true;
        }
    }
    false
}

pub fn run() {
    let start = std::time::Instant::now();

    let input = fs::read_to_string("./src/input/day07.txt").expect("Failed to read input file");

    let equations: Vec<(i64, Vec<i64>)> = input
    .lines()
    .map(|line| {
        let parts: Vec<&str> = line.split(": ").collect();
        let target = parts[0].parse::<i64>().unwrap();
        let numbers = parts[1].split_whitespace()
                               .map(|n| n.parse::<i64>().unwrap())
                               .collect();
        (target, numbers)
    })
    .collect();

    println!("Day 7 - Part 1: {}", part1(&equations));
    let part1_duration = start.elapsed();
    println!("Day 7 - Part 1 duration: {:?}", part1_duration);

    println!("Day 7 - Part 2: {}", part2(&equations));
    let part2_duration = start.elapsed() - part1_duration;
    println!("Day 7 - Part 2 duration: {:?}", part2_duration);

    println!("Day 7 - Total duration: {:?}", start.elapsed());
}
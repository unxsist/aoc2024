use std::{collections::{HashMap, HashSet}, fs};

pub fn part1(rules: &HashMap<i32, HashSet<i32>>, updates: &Vec<Vec<i32>>) -> i32 {
    let mut total_middle_sum = 0;

    for update in updates {
        if is_update_valid(&update, rules) {
            if let Some(middle) = find_middle(&update) {
                total_middle_sum += middle;
            }
        }
    }

    total_middle_sum
}

fn parse_input(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let mut parts = input.split("\n\n");
    let rules_section = parts.next().unwrap();
    let updates_section = parts.next().unwrap();

    let mut rules = HashMap::new();
    for line in rules_section.lines() {
        let (x, y) = line.split_once('|').unwrap();
        let x: i32 = x.parse().unwrap();
        let y: i32 = y.parse().unwrap();
        rules.entry(x).or_insert_with(HashSet::new).insert(y);
    }

    let updates = updates_section
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    (rules, updates)
}

fn is_update_valid(update: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut index_map = HashMap::new();
    for (i, &page) in update.iter().enumerate() {
        index_map.insert(page, i);
    }

    for (&x, ys) in rules.iter() {
        if let Some(&xi) = index_map.get(&x) {
            for &y in ys {
                if let Some(&yi) = index_map.get(&y) {
                    if xi >= yi {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn find_middle(update: &Vec<i32>) -> Option<i32> {
    if update.is_empty() {
        None
    } else {
        Some(update[update.len() / 2])
    }
}

pub fn part2(rules: &HashMap<i32, HashSet<i32>>, updates: &Vec<Vec<i32>>) -> i32 {
    let mut total_middle_sum = 0;

    for update in updates {
        if !is_update_valid(&update, rules) {
            let reordered = reorder_update(&update, &rules);
            if let Some(middle) = find_middle(&reordered) {
                total_middle_sum += middle;
            }
        }
    }

    total_middle_sum
}

fn reorder_update(update: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut graph = HashMap::new();
    let mut in_degree = HashMap::new();

    for &page in update {
        graph.entry(page).or_insert_with(Vec::new);
        in_degree.entry(page).or_insert(0);
    }

    for (&x, ys) in rules {
        if update.contains(&x) {
            for &y in ys {
                if update.contains(&y) {
                    graph.entry(x).or_insert_with(Vec::new).push(y);
                    *in_degree.entry(y).or_insert(0) += 1;
                }
            }
        }
    }

    let mut sorted = Vec::new();
    let mut queue: Vec<i32> = in_degree
        .iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(&page, _)| page)
        .collect();

    while let Some(page) = queue.pop() {
        sorted.push(page);
        if let Some(dependents) = graph.get(&page) {
            for &dependent in dependents {
                if let Some(deg) = in_degree.get_mut(&dependent) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push(dependent);
                    }
                }
            }
        }
    }

    sorted
}

pub fn run() {
    let start = std::time::Instant::now();

    let input = fs::read_to_string("./src/input/day05.txt").expect("Failed to read input file");
    let (rules, updates) = parse_input(&input);

    println!("Day 5 - Part 1: {}", part1(&rules, &updates));
    let part1_duration = start.elapsed();
    println!("Day 5 - Part 1 duration: {:?}", part1_duration);

    println!("Day 5 - Part 2: {}", part2(&rules, &updates));
    let part2_duration = start.elapsed() - part1_duration;
    println!("Day 5 - Part 2 duration: {:?}", part2_duration);

    println!("Day 5 - Total duration: {:?}", start.elapsed());
}
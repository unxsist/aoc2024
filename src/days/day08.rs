use std::{collections::{HashMap, HashSet}, fs};

fn parse_input(input: &str) -> Vec<(i32, i32, char)> {
    let mut antennas = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch.is_alphanumeric() {
                antennas.push((x as i32, y as i32, ch));
            }
        }
    }
    antennas
}

fn part1(map_width: i32, map_height: i32, antennas: &Vec<(i32, i32, char)>) -> usize {
    let mut antinodes = HashSet::new();

    for (i, (x1, y1, freq1)) in antennas.iter().enumerate() {
        for (x2, y2, freq2) in &antennas[i + 1..] {
            if freq1 != freq2 {
                continue;
            }

            let nodes = calculate_antinodes(*x1, *y1, *x2, *y2);
            
            for (x, y) in nodes {
                if x >= 0 && x < map_width && y >= 0 && y < map_height {
                    antinodes.insert((x, y));
                }
            }
        }
    }
    
    antinodes.len()
}

fn calculate_antinodes(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {
    let mut antinodes = Vec::new();
    
    let dx = x2 - x1;
    let dy = y2 - y1;
    
    for t in -10..=10 {
        let x = x1 + (dx * t) / 3;  
        let y = y1 + (dy * t) / 3;
        
        let d1_squared = (x - x1).pow(2) + (y - y1).pow(2);
        let d2_squared = (x - x2).pow(2) + (y - y2).pow(2);
        
        if d1_squared * 4 == d2_squared || d2_squared * 4 == d1_squared {
            antinodes.push((x, y));
        }
    }

    antinodes
}

fn part2(map_width: i32, map_height: i32, antennas: &Vec<(i32, i32, char)>) -> usize {
    let mut antinodes = HashSet::new();

    let mut freq_groups: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for &(x, y, freq) in antennas {
        freq_groups.entry(freq).or_default().push((x, y));
    }

    for (_freq, positions) in freq_groups.iter() {
        if positions.len() < 2 {
            continue;
        }

        for &(ax, ay) in positions {
            let mut is_collinear_with_others = false;
            
            for i in 0..positions.len() {
                for j in i+1..positions.len() {
                    let (x1, y1) = positions[i];
                    let (x2, y2) = positions[j];
                    
                    if (ax == x1 && ay == y1) || (ax == x2 && ay == y2) {
                        continue;
                    }

                    if is_collinear(x1, y1, x2, y2, ax, ay) {
                        is_collinear_with_others = true;
                        break;
                    }
                }
                if is_collinear_with_others {
                    break;
                }
            }

            if is_collinear_with_others {
                antinodes.insert((ax, ay));
            }
        }

        for i in 0..positions.len() {
            for j in i+1..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];
                
                for y in 0..map_height {
                    for x in 0..map_width {
                        if is_collinear(x1, y1, x2, y2, x, y) {
                            antinodes.insert((x, y));
                        }
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn is_collinear(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> bool {
    (x2 - x1) * (y3 - y1) == (y2 - y1) * (x3 - x1)
}

pub fn run() {
    let start = std::time::Instant::now();

    let input = fs::read_to_string("./src/input/day08.txt").expect("Failed to read input file");

    let map_width = input.lines().next().unwrap().len() as i32;
    let map_height = input.lines().count() as i32;

    let antennas = parse_input(&input);

    println!("Day 8 - Part 1: {}", part1(map_width, map_height, &antennas));
    let part1_duration = start.elapsed();
    println!("Day 8 - Part 1 duration: {:?}", part1_duration);

    println!("Day 8 - Part 2: {}", part2(map_width, map_height, &antennas));
    let part2_duration = start.elapsed() - part1_duration;
    println!("Day 8 - Part 2 duration: {:?}", part2_duration);

    println!("Day 8 - Total duration: {:?}", start.elapsed());
}
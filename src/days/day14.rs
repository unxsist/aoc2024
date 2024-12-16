use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

impl Robot {
    fn parse(line: &str) -> Self {
        let parts: Vec<&str> = line.split(' ').collect();
        let pos_parts: Vec<i64> = parts[0][2..]
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        let vel_parts: Vec<i64> = parts[1][2..]
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        Robot {
            position: (pos_parts[0], pos_parts[1]),
            velocity: (vel_parts[0], vel_parts[1]),
        }
    }

    fn update_position(&mut self, width: i64, height: i64) {
        self.position.0 = (self.position.0 + self.velocity.0).rem_euclid(width);
        self.position.1 = (self.position.1 + self.velocity.1).rem_euclid(height);
    }
}

fn part1(input: &str) -> i64 {
    let mut robots: Vec<Robot> = input.lines().map(Robot::parse).collect();
    
    simulate_robots(&mut robots, 100, 101, 103);
    
    let (q1, q2, q3, q4) = count_robots_in_quadrants(&robots, 101, 103);
    q1 * q2 * q3 * q4
}

fn simulate_robots(robots: &mut Vec<Robot>, steps: i64, width: i64, height: i64) {
    for _ in 0..steps {
        for robot in robots.iter_mut() {
            robot.update_position(width, height);
        }
    }
}

fn count_robots_in_quadrants(robots: &[Robot], width: i64, height: i64) -> (i64, i64, i64, i64) {
    let mid_x = width / 2;
    let mid_y = height / 2;
    let mut counts = (0, 0, 0, 0);

    for robot in robots {
        let (x, y) = robot.position;
        
        if x == mid_x || y == mid_y {
            continue;
        }

        match (x < mid_x, y < mid_y) {
            (true, true) => counts.0 += 1,
            (false, true) => counts.1 += 1,
            (true, false) => counts.2 += 1,
            (false, false) => counts.3 += 1,
        }
    }

    counts
}

fn part2(input: &str) -> i64 {
    let mut robots: Vec<Robot> = input.lines().map(Robot::parse).collect();
    // brute force this shit and write stdout to file...
    simulate_and_print(&mut robots, 10000, 101, 103);
    0
}

fn simulate_and_print(robots: &mut Vec<Robot>, steps: i64, width: i64, height: i64) {
    for step in 0..steps {
        print_grid(robots, width, height, step);
        
        for robot in robots.iter_mut() {
            robot.update_position(width, height);
        }
    }

    print_grid(robots, width, height, steps);
}

fn print_grid(robots: &[Robot], width: i64, height: i64, step: i64) {
    println!("\nStep {}:", step);
    
    let mut positions: HashMap<(i64, i64), i64> = HashMap::new();
    for robot in robots {
        *positions.entry(robot.position).or_default() += 1;
    }
    
    for y in 0..height {
        for x in 0..width {
            match positions.get(&(x, y)) {
                Some(count) => print!("{}", count),
                None => print!("."),
            }
        }
        println!();
    }
    println!();
}

pub fn run() {
    let start = std::time::Instant::now();

    let input = fs::read_to_string("./src/input/day14.txt").expect("Failed to read input file");

    println!("Day 14 - Part 1: {}", part1(&input));
    let part1_duration = start.elapsed();
    println!("Day 14 - Part 1 duration: {:?}", part1_duration);

    println!("Day 14 - Part 2: {}", part2(&input));
    let part2_duration = start.elapsed() - part1_duration;
    println!("Day 14 - Part 2 duration: {:?}", part2_duration);

    println!("Day 14 - Total duration: {:?}", start.elapsed());
}

use std::{error::Error, fs};

#[derive(Debug)]
struct ClawMachine {
    button_a: (u32, u32),
    button_b: (u32, u32),
    prize: (u32, u32),
}

fn parse_numbers(s: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let s = s.trim();
    let s = s.split(": ").nth(1).ok_or("Invalid format")?;
    
    let parts: Vec<&str> = s.split(", ").collect();
    if parts.len() != 2 {
        return Err("Invalid coordinates format".into());
    }

    let x_part = parts[0].trim();
    let y_part = parts[1].trim();

    let x = if x_part.starts_with("X=") {
        x_part[2..].parse::<u32>()?
    } else if x_part.starts_with("X+") {
        x_part[2..].parse()?
    } else {
        return Err("Invalid X format".into());
    };

    let y = if y_part.starts_with("Y=") {
        y_part[2..].parse::<u32>()?
    } else if y_part.starts_with("Y+") {
        y_part[2..].parse()?
    } else {
        return Err("Invalid Y format".into());
    };

    Ok((x, y))
}

fn parse_input(input: &str) -> Result<Vec<ClawMachine>, Box<dyn Error>> {
    let mut machines = Vec::new();
    let mut current_machine = None;
    
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        if line.starts_with("Button A:") {
            let (x, y) = parse_numbers(line)?;
            current_machine = Some((
                (x, y),
                (0, 0),
                (0, 0)
            ));
        } else if line.starts_with("Button B:") && current_machine.is_some() {
            let (x, y) = parse_numbers(line)?;
            if let Some(mut machine) = current_machine {
                machine.1 = (x, y);
                current_machine = Some(machine);
            }
        } else if line.starts_with("Prize:") && current_machine.is_some() {
            let (x, y) = parse_numbers(line)?;
            if let Some(mut machine) = current_machine.take() {
                machine.2 = (x, y);
                machines.push(ClawMachine {
                    button_a: machine.0,
                    button_b: machine.1,
                    prize: machine.2,
                });
            }
        }
    }
    
    Ok(machines)
}

fn part1(machines: Vec<ClawMachine>) -> i64 {
    let mut total_tokens = 0;

    for machine in &machines {
        let (n1, n2) = linear((machine.button_a, machine.button_b), machine.prize, 0);

        if n1.fract() == 0.0
            && n2.fract() == 0.0
            && (0..100).contains(&(n1 as i32))
            && (0..100).contains(&(n2 as i32))
        {
            total_tokens += calculate_tokens(n1 as i64, n2 as i64)
        }
    }
    
    total_tokens
}

fn part2(machines: Vec<ClawMachine>) -> i64 {
    let mut total_tokens = 0;
    for machine in &machines {
        let (n1, n2) = linear((machine.button_a, machine.button_b), machine.prize, 10000000000000);
        if n1.fract() == 0.0 && n2.fract() == 0.0 {
            total_tokens += calculate_tokens(n1 as i64, n2 as i64);
        }
    }
    total_tokens
}

fn linear(
    buttons: ((u32, u32), (u32, u32)),
    prize: (u32, u32),
    delta: u64,
) -> (f64, f64) {
    let ((button_a_x, button_a_y), (button_b_x, button_b_y)) = buttons;
    let (prize_x, prize_y) = prize;

    let a_x = button_a_x as f64;
    let a_y = button_a_y as f64;
    let b_x = button_b_x as f64;
    let b_y = button_b_y as f64;
    let prize_x = prize_x as f64 + delta as f64;
    let prize_y = prize_y as f64 + delta as f64;

    let determinant = a_x * b_y - a_y * b_x;

    let n1 = (prize_x * b_y - prize_y * b_x) / determinant;
    let n2 = (prize_y * a_x - prize_x * a_y) / determinant;

    (n1, n2)
}

fn calculate_tokens(a_presses: i64, b_presses: i64) -> i64 {
    a_presses * 3 + b_presses * 1
}

pub fn run() {
    let start = std::time::Instant::now();

    let input = fs::read_to_string("./src/input/day13.txt").expect("Failed to read input file");
    let m_p1 = parse_input(&input).expect("Failed to parse input");
    let m_p2 = parse_input(&input).expect("Failed to parse input");

    println!("Day 13 - Part 1: {}", part1(m_p1));
    let part1_duration = start.elapsed();
    println!("Day 13 - Part 1 duration: {:?}", part1_duration);

    println!("Day 13 - Part 2: {}", part2(m_p2));
    let part2_duration = start.elapsed() - part1_duration;
    println!("Day 13 - Part 2 duration: {:?}", part2_duration);

    println!("Day 13 - Total duration: {:?}", start.elapsed());
}

use std::fs;

fn read_input() -> Vec<Vec<char>> {
    fs::read_to_string("./src/input/day04.txt")
        .expect("Failed to read the input file") 
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn part1() -> usize {
    let grid = read_input();
    let directions = [
        (0, 1),  // Right
        (1, 0),  // Down
        (1, 1),  // Down-Right
        (1, -1), // Down-Left
        (0, -1), // Left
        (-1, 0), // Up
        (-1, -1), // Up-Left
        (-1, 1),  // Up-Right
    ];

    let word = "XMAS";
    let rows = grid.len();
    let cols = grid[0].len();
    let word_len = word.len();
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            for &(dx, dy) in &directions {
                if is_match(&grid, word, row as isize, col as isize, dx, dy, rows, cols, word_len) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn is_match(
    grid: &[Vec<char>],
    word: &str,
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    rows: usize,
    cols: usize,
    word_len: usize,
) -> bool {
    let chars: Vec<char> = word.chars().collect();
    for i in 0..word_len {
        let nx = x + i as isize * dx;
        let ny = y + i as isize * dy;

        if nx < 0 || ny < 0 || nx >= rows as isize || ny >= cols as isize {
            return false;
        }

        if grid[nx as usize][ny as usize] != chars[i] {
            return false;
        }
    }
    true
}

pub fn part2() -> usize {
    let grid = read_input();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for row in 1..rows-1 {
        for col in 1..cols-1 {
            if grid[row][col] != 'A' {
                continue;
            }
            
            let positions = [
                (-1, -1), (-1, 1),  // Upper positions
                (1, -1), (1, 1)     // Lower positions
            ];
            
            for i in 0..4 {
                for j in i+1..4 {
                    let (r1, c1) = positions[i];
                    let (r2, c2) = positions[j];
                    
                    if r1 + r2 == 0 && c1 + c2 == 0 {
                        continue;
                    }
                    
                    let pos1_row = (row as i32 + r1) as usize;
                    let pos1_col = (col as i32 + c1) as usize;
                    let pos2_row = (row as i32 + r2) as usize;
                    let pos2_col = (col as i32 + c2) as usize;
                    
                    if pos1_row >= rows || pos1_col >= cols || 
                       pos2_row >= rows || pos2_col >= cols {
                        continue;
                    }
                    
                    if grid[pos1_row][pos1_col] == 'M' && 
                       grid[pos2_row][pos2_col] == 'M' {
                        let s1_row = (row as i32 - r1) as usize;
                        let s1_col = (col as i32 - c1) as usize;
                        let s2_row = (row as i32 - r2) as usize;
                        let s2_col = (col as i32 - c2) as usize;
                        
                        if s1_row < rows && s1_col < cols && 
                           s2_row < rows && s2_col < cols && 
                           grid[s1_row][s1_col] == 'S' && 
                           grid[s2_row][s2_col] == 'S' {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count
}

pub fn run() {
    let start = std::time::Instant::now();

    println!("Day 4 - Part 1: {}", part1());
    let part1_duration = start.elapsed();
    println!("Day 4 - Part 1 duration: {:?}", part1_duration);

    println!("Day 4 - Part 2: {}", part2());
    let part2_duration = start.elapsed() - part1_duration;
    println!("Day 4 - Part 2 duration: {:?}", part2_duration);

    println!("Day 4 - Total duration: {:?}", start.elapsed());
}
use std::{collections::{HashMap, HashSet, VecDeque}, fs};

fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    let trailheads = find_trailheads(&grid);
    
    trailheads.iter()
        .map(|&pos| calculate_trailhead_score(&grid, pos))
        .sum()
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|line| 
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        )
        .collect()
}

fn find_trailheads(grid: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();
    
    for (y, row) in grid.iter().enumerate() {
        for (x, &height) in row.iter().enumerate() {
            if height == 0 {
                trailheads.push((x, y));
            }
        }
    }
    
    trailheads
}

fn calculate_trailhead_score(grid: &[Vec<u8>], start: (usize, usize)) -> usize {
    let mut reachable_nines = HashSet::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    queue.push_back((start, 0));
    visited.insert(start);
    
    while let Some(((x, y), current_height)) = queue.pop_front() {
        if grid[y][x] == 9 {
            reachable_nines.insert((x, y));
        }
        
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            
            if new_x < 0 || new_y < 0 || 
               new_x >= grid[0].len() as i32 || 
               new_y >= grid.len() as i32 {
                continue;
            }
            
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            let new_pos = (new_x, new_y);
            
            if !visited.contains(&new_pos) && 
               grid[new_y][new_x] as i32 == current_height + 1 {
                visited.insert(new_pos);
                queue.push_back((new_pos, current_height + 1));
            }
        }
    }
    
    reachable_nines.len()
}

fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    let trailheads = find_trailheads(&grid);
    
    trailheads.iter()
        .map(|&pos| count_distinct_trails(&grid, pos))
        .sum()
}

fn count_distinct_trails(grid: &[Vec<u8>], start: (usize, usize)) -> usize {
    let mut memo: HashMap<(usize, usize, u8), usize> = HashMap::new();
    
    fn dfs(grid: &[Vec<u8>], 
           pos: (usize, usize), 
           current_height: u8,
           memo: &mut HashMap<(usize, usize, u8), usize>) -> usize {
        
        if grid[pos.1][pos.0] == 9 {
            return 1;
        }
        
        let key = (pos.0, pos.1, current_height);
        if let Some(&count) = memo.get(&key) {
            return count;
        }
        
        let mut total_paths = 0;
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
        for (dx, dy) in directions {
            let new_x = pos.0 as i32 + dx;
            let new_y = pos.1 as i32 + dy;
            
            if new_x < 0 || new_y < 0 || 
               new_x >= grid[0].len() as i32 || 
               new_y >= grid.len() as i32 {
                continue;
            }
            
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            
            if grid[new_y][new_x] as i32 == current_height as i32 + 1 {
                total_paths += dfs(grid, (new_x, new_y), current_height + 1, memo);
            }
        }
        
        memo.insert(key, total_paths);
        total_paths
    }
    
    dfs(grid, start, grid[start.1][start.0], &mut memo)
}

pub fn run() {
    let start = std::time::Instant::now();

    let input = fs::read_to_string("./src/input/day10.txt").expect("Failed to read input file");

    println!("Day 10 - Part 1: {}", part1(&input));
    let part1_duration = start.elapsed();
    println!("Day 10 - Part 1 duration: {:?}", part1_duration);

    println!("Day 10 - Part 2: {}", part2(&input));
    let part2_duration = start.elapsed() - part1_duration;
    println!("Day 10 - Part 2 duration: {:?}", part2_duration);

    println!("Day 10 - Total duration: {:?}", start.elapsed());
}
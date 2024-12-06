use std::{collections::{HashMap, HashSet}, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_forward(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => x.checked_sub(1).map(|nx| (nx, y)),
            Direction::Down => Some((x + 1, y)),
            Direction::Left => y.checked_sub(1).map(|ny| (x, ny)),
            Direction::Right => Some((x, y + 1)),
        }
    }
}

pub fn part1() -> usize {
    let input = fs::read_to_string("./src/input/day06.txt").expect("Failed to read input file");

    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut position = (0, 0);
    let mut direction = Direction::Up;

    for (x, row) in grid.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            if "^v<>".contains(cell) {
                position = (x, y);
                direction = match cell {
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => unreachable!(),
                };
                break;
            }
        }
    }

    grid[position.0][position.1] = '.';

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(position);

    loop {
        let next_position = direction.move_forward(position);
        let (nx, ny) = next_position.unwrap();

        if nx >= rows || ny >= cols || grid.get(nx).and_then(|row| row.get(ny)).is_none() {
            break;
        }

        if grid[nx][ny] == '#' {
            direction = direction.turn_right();
        } else {
            position = (nx, ny);
            visited.insert(position);
        }
    }

    visited.len()
}

fn part2() -> usize {
    let input = fs::read_to_string("./src/input/day06.txt").expect("Failed to read input file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    
    let mut start_position = (0, 0);
    let mut start_direction = Direction::Up;
    
    'outer: for (x, row) in grid.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            if let Some(dir) = match cell {
                '^' => Some(Direction::Up),
                'v' => Some(Direction::Down),
                '<' => Some(Direction::Left),
                '>' => Some(Direction::Right),
                _ => None,
            } {
                start_position = (x, y);
                start_direction = dir;
                break 'outer;
            }
        }
    }

    fn is_valid_position(pos: (usize, usize), grid: &[Vec<char>]) -> bool {
        pos.0 < grid.len() && pos.1 < grid[0].len()
    }

    fn simulate_with_obstruction(
        grid: &[Vec<char>],
        start_pos: (usize, usize),
        start_dir: Direction,
        obstruction: (usize, usize),
    ) -> bool {
        let mut pos = start_pos;
        let mut dir = start_dir;
        let mut visited = HashSet::new();
        let mut state_history = vec![];
        
        visited.insert((pos, dir));
        state_history.push((pos, dir));
        
        let max_steps = grid.len() * grid[0].len() * 4;
        
        for _ in 0..max_steps {
            let next_pos = match dir.move_forward(pos) {
                Some(np) if is_valid_position(np, grid) => np,
                _ => return false,
            };
            
            if next_pos == obstruction || grid[next_pos.0][next_pos.1] == '#' {
                dir = dir.turn_right();
                
                if visited.contains(&(pos, dir)) {
                    if let Some(loop_start) = state_history.iter().position(|&x| x == (pos, dir)) {
                        let loop_length = state_history.len() - loop_start;
                        return loop_length > 1;
                    }
                }
                
                visited.insert((pos, dir));
                state_history.push((pos, dir));
            } else {
                pos = next_pos;
                
                if visited.contains(&(pos, dir)) {
                    if let Some(loop_start) = state_history.iter().position(|&x| x == (pos, dir)) {
                        let loop_length = state_history.len() - loop_start;
                        return loop_length > 1;
                    }
                }
                
                visited.insert((pos, dir));
                state_history.push((pos, dir));
            }
        }
        
        false
    }

    let mut valid_positions = 0;

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if (x, y) != start_position && grid[x][y] != '#' {
                if simulate_with_obstruction(&grid, start_position, start_direction, (x, y)) {
                    valid_positions += 1;
                }
            }
        }
    }

    valid_positions
}

pub fn run() {
    let start = std::time::Instant::now();

    println!("Day 6 - Part 1: {}", part1());
    let part1_duration = start.elapsed();
    println!("Day 6 - Part 1 duration: {:?}", part1_duration);

    println!("Day 6 - Part 2: {}", part2());
    let part2_duration = start.elapsed() - part1_duration;
    println!("Day 6 - Part 2 duration: {:?}", part2_duration);

    println!("Day 6 - Total duration: {:?}", start.elapsed());
}
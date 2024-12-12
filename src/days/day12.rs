use std::{collections::{HashSet, VecDeque}, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: i64,
    col: i64,
}

fn part1(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len() as i64;
    let cols = grid[0].len() as i64;
    let mut visited = HashSet::new();
    let mut total_price = 0;

    for row in 0..rows {
        for col in 0..cols {
            let point = Point { row, col };
            if !visited.contains(&point) {
                let plant_type = grid[row as usize][col as usize];
                let (area, perimeter) = find_region_with_perimeter(&grid, point, plant_type, &mut visited);
                total_price += area * perimeter;
            }
        }
    }

    total_price
}

fn find_region_with_perimeter(grid: &Vec<Vec<char>>, start: Point, plant_type: char, visited: &mut HashSet<Point>) -> (i64, i64) {
    let mut queue = VecDeque::new();
    let mut region = HashSet::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    queue.push_back(start);
    region.insert(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        for (dx, dy) in directions.iter() {
            let next = Point {
                row: current.row + dx,
                col: current.col + dy,
            };

            if within_bounds(next, grid) &&
               !visited.contains(&next) &&
               grid[next.row as usize][next.col as usize] == plant_type {
                queue.push_back(next);
                region.insert(next);
                visited.insert(next);
            }
        }
    }

    let area = region.len() as i64;
    let perimeter = calculate_perimeter(grid, &region);

    (area, perimeter)
}

fn calculate_perimeter(grid: &Vec<Vec<char>>, region: &HashSet<Point>) -> i64 {
    let mut perimeter = 0;
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for &point in region {
        for (dx, dy) in directions.iter() {
            let neighbor = Point {
                row: point.row + dx,
                col: point.col + dy,
            };

            if !within_bounds(neighbor, grid) || !region.contains(&neighbor) {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn within_bounds(point: Point, grid: &Vec<Vec<char>>) -> bool {
    point.row >= 0 && point.col >= 0 &&
    point.row < grid.len() as i64 && point.col < grid[0].len() as i64
}

fn part2(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut regions = Vec::new();
    let mut visited = HashSet::new();

    for row in 0..grid.len() as i64 {
        for col in 0..grid[0].len() as i64 {
            let point = Point { row, col };
            if !visited.contains(&point) {
                let plant_type = grid[row as usize][col as usize];
                let region = find_region(&grid, point, plant_type, &mut visited);
                regions.push(region);
            }
        }
    }

    let mut total_price = 0;
    for region in &regions {
        let area = region.len() as i64;
        let sides = calculate_sides(&grid, region);
        total_price += area * sides;
    }

    total_price
}

fn find_region(grid: &Vec<Vec<char>>, start: Point, plant_type: char, visited: &mut HashSet<Point>) -> HashSet<Point> {
    let mut queue = Vec::new();
    let mut region = HashSet::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    queue.push(start);
    visited.insert(start);

    while let Some(current) = queue.pop() {
        region.insert(current);

        for (dx, dy) in directions.iter() {
            let neighbor = Point {
                row: current.row + dx,
                col: current.col + dy,
            };

            if within_bounds(neighbor, grid) &&
               !visited.contains(&neighbor) &&
               grid[neighbor.row as usize][neighbor.col as usize] == plant_type {
                queue.push(neighbor);
                visited.insert(neighbor);
            }
        }
    }

    region
}

fn calculate_sides(grid: &Vec<Vec<char>>, region: &HashSet<Point>) -> i64 {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut edges = HashSet::new();

    for &point in region {
        for (idx, (dr, dc)) in directions.iter().enumerate() {
            let neighbor = Point {
                row: point.row + *dr,
                col: point.col + *dc,
            };

            if !within_bounds(neighbor, grid) || !region.contains(&neighbor) {
                edges.insert((point.row, point.col, idx));
            }
        }
    }

    let mut sides = 0;
    let mut visited_edges = HashSet::new();

    for &(row, col, dir) in &edges {
        if visited_edges.contains(&(row, col, dir)) {
            continue;
        }

        sides += 1;

        let perp_dirs = match dir {
            0 | 2 => [(1, 0), (-1, 0)],
            1 | 3 => [(0, 1), (0, -1)],
            _ => unreachable!(),
        };

        for &(dr, dc) in &perp_dirs {
            let mut curr_r = row;
            let mut curr_c = col;

            loop {
                curr_r += dr;
                curr_c += dc;

                if curr_r < 0 || curr_r >= grid.len() as i64
                    || curr_c < 0 || curr_c >= grid[0].len() as i64
                {
                    break;
                }

                let curr_edge = (curr_r, curr_c, dir);
                if !edges.contains(&curr_edge) {
                    break;
                }

                visited_edges.insert(curr_edge);
            }
        }
    }

    sides
}

pub fn run() {
    let start = std::time::Instant::now();

    let input = fs::read_to_string("./src/input/day12.txt").expect("Failed to read input file");

    println!("Day 12 - Part 1: {}", part1(&input));
    let part1_duration = start.elapsed();
    println!("Day 12 - Part 1 duration: {:?}", part1_duration);

    println!("Day 12 - Part 2: {}", part2(&input));
    let part2_duration = start.elapsed() - part1_duration;
    println!("Day 12 - Part 2 duration: {:?}", part2_duration);

    println!("Day 12 - Total duration: {:?}", start.elapsed());
}
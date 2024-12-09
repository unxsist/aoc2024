use std::fs;

fn part1(input: &str) -> i64 {
    let mut disk = Vec::new();
    let numbers: Vec<usize> = input.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    
    let mut file_id = 0;
    for (i, &count) in numbers.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..count {
                disk.push(Some(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..count {
                disk.push(None);
            }
        }
    }

    loop {
        let mut moved = false;
        if let Some(right_pos) = disk.iter().rposition(|&x| x.is_some()) {
            if let Some(left_pos) = disk[..right_pos].iter().position(|&x| x.is_none()) {
                disk[left_pos] = disk[right_pos];
                disk[right_pos] = None;
                moved = true;
            }
        }
        if !moved {
            break;
        }
    }

    disk.iter()
        .enumerate()
        .filter_map(|(pos, &block)| {
            block.map(|id| pos as i64 * id as i64)
        })
        .sum()
}

fn part2(input: &str) -> i64 {
     let mut disk = Vec::new();
     let numbers: Vec<usize> = input.chars()
         .map(|c| c.to_digit(10).unwrap() as usize)
         .collect();
     
     let mut file_id = 0;
     let mut file_sizes = Vec::new();
     
     for (i, &count) in numbers.iter().enumerate() {
         if i % 2 == 0 {
             file_sizes.push((file_id, count));
             for _ in 0..count {
                 disk.push(Some(file_id));
             }
             file_id += 1;
         } else {
             for _ in 0..count {
                 disk.push(None);
             }
         }
     }
 
     file_sizes.sort_by_key(|&(id, _)| std::cmp::Reverse(id));
 
     for (file_id, size) in file_sizes {
         let start_pos = disk.iter().position(|&x| x == Some(file_id)).unwrap();
         let end_pos = start_pos + size;
         
         let mut best_pos = start_pos;
         let mut current_free = 0;
         let mut potential_pos = None;
 
         for (pos, &block) in disk[..start_pos].iter().enumerate() {
             if block.is_none() {
                 if current_free == 0 {
                     potential_pos = Some(pos);
                 }
                 current_free += 1;
                 if current_free >= size {
                     best_pos = potential_pos.unwrap();
                     break;
                 }
             } else {
                 current_free = 0;
                 potential_pos = None;
             }
         }

         if best_pos < start_pos {
             let file: Vec<_> = disk[start_pos..end_pos].to_vec();

             for pos in start_pos..end_pos {
                 disk[pos] = None;
             }

             for (offset, &value) in file.iter().enumerate() {
                 disk[best_pos + offset] = value;
             }
         }
     }
 
     disk.iter()
         .enumerate()
         .filter_map(|(pos, &block)| block.map(|id| pos as i64 * id as i64))
         .sum()
}

pub fn run() {
    let start = std::time::Instant::now();

    let input = fs::read_to_string("./src/input/day09.txt").expect("Failed to read input file");

    println!("Day 9 - Part 1: {}", part1(&input));
    let part1_duration = start.elapsed();
    println!("Day 9 - Part 1 duration: {:?}", part1_duration);

    println!("Day 9 - Part 2: {}", part2(&input));
    let part2_duration = start.elapsed() - part1_duration;
    println!("Day 9 - Part 2 duration: {:?}", part2_duration);

    println!("Day 9 - Total duration: {:?}", start.elapsed());
}
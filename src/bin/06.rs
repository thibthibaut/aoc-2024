use indicatif::ParallelProgressIterator;
use itertools::{iproduct, Itertools};
use rayon::prelude::*;
use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    // Locate the ^
    let (starting_pos, _) = map
        .iter()
        .flatten()
        .find_position(|x| **x == '^')
        .expect("Did not find the the starting position :(");

    // Convert to a (x, y) position
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    let starting_pos = ((starting_pos as i32 % width), (starting_pos as i32 / width));

    let directions: [(i32, i32); 4] = [
        (0, -1), // up
        (1, 0),  // right
        (0, 1),  // down
        (-1, 0), // left
    ];

    let mut current_pos = starting_pos;
    let mut direction_switch = 0;
    loop {
        // Mark the current with a X
        map[current_pos.1 as usize][current_pos.0 as usize] = 'X';

        // Get the current direction
        let current_dir = directions[direction_switch % 4];
        // Update the current position
        let new_pos = (current_pos.0 + current_dir.0, current_pos.1 + current_dir.1);
        // exit if we are out of the map
        if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= width || new_pos.1 >= height {
            break;
        }

        // Get the current char
        let current_char = map[new_pos.1 as usize][new_pos.0 as usize];
        // Switch the direction if hit something
        if current_char == '#' {
            direction_switch += 1;
        } else {
            // Update the current position
            current_pos = new_pos;
        }
    }

    Some(map.iter().flatten().filter(|x| **x == 'X').count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    // Locate the '^'
    let (starting_pos, _) = map
        .iter()
        .flatten()
        .find_position(|&&x| x == '^')
        .expect("Did not find the starting position :(");

    // Convert to a (x, y) position
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    let starting_pos = (starting_pos as i32 % width, starting_pos as i32 / width);
    let directions: [(i32, i32); 4] = [
        (0, -1), // up
        (1, 0),  // right
        (0, 1),  // down
        (-1, 0), // left
    ];

    // Precompute valid crate positions
    let crate_positions: Vec<_> = iproduct!(0..width, 0..height)
        .filter(|&crate_pos| {
            crate_pos != starting_pos && map[crate_pos.1 as usize][crate_pos.0 as usize] != '#'
        })
        .collect();

    let total_pos = crate_positions.len() as u64;
    let loop_count = crate_positions
        .into_par_iter() // Parallel iterator
        .progress_count(total_pos)
        .map(|crate_pos| {
            // Clone the map for each thread
            let mut local_map = map.clone();
            // Add the crate to the map
            local_map[crate_pos.1 as usize][crate_pos.0 as usize] = '#';

            let mut current_pos = starting_pos;
            let mut direction_switch = 0;
            let mut visited: HashSet<((i32, i32), usize)> = HashSet::new();
            let mut in_loop = 0;

            loop {
                // Check if the current position and direction have been visited
                if !visited.insert((current_pos, direction_switch % 4)) {
                    in_loop = 1; // Loop detected
                    break;
                }

                // Calculate the new position based on the current direction
                let current_dir = directions[direction_switch % 4];
                let new_pos = (current_pos.0 + current_dir.0, current_pos.1 + current_dir.1);

                // Exit if out of bounds
                if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= width || new_pos.1 >= height {
                    break;
                }

                // Get the current character
                let current_char = local_map[new_pos.1 as usize][new_pos.0 as usize];
                if current_char == '#' {
                    // Switch direction if we hit an obstacle
                    direction_switch += 1;
                } else {
                    // Update the current position
                    current_pos = new_pos;
                }
            }

            in_loop
        })
        .sum();

    Some(loop_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

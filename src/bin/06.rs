use std::collections::HashSet;

use rayon::prelude::*;

use itertools::Itertools;

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

    let mut loop_count = 0;
    // Generate all the possible positions to put a crate
    for crate_pos in (0..width).cartesian_product(0..height) {
        if crate_pos == starting_pos {
            continue;
        }
        if map[crate_pos.1 as usize][crate_pos.0 as usize] == '#' {
            continue;
        }
        // add the crate to the map
        map[crate_pos.1 as usize][crate_pos.0 as usize] = '#';

        let mut current_pos = starting_pos;
        let mut direction_switch = 0;
        let mut visited: HashSet<((i32, i32), usize)> = HashSet::new();
        loop {
            // Check if we are in a loop: same position and direction
            if !visited.insert((current_pos, direction_switch % 4)) {
                loop_count += 1;
                break;
            }

            // Mark the current position with 'X'
            map[current_pos.1 as usize][current_pos.0 as usize] = 'X';

            // Get the current direction
            let current_dir = directions[direction_switch % 4];
            // Update the current position
            let new_pos = (current_pos.0 + current_dir.0, current_pos.1 + current_dir.1);

            // Exit if we are out of bounds
            if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= width || new_pos.1 >= height {
                break;
            }

            // Get the current character
            let current_char = map[new_pos.1 as usize][new_pos.0 as usize];
            if current_char == '#' {
                // Switch direction if we hit an obstacle
                direction_switch += 1;
            } else {
                // Update the current position
                current_pos = new_pos;
            }
        }

        map[crate_pos.1 as usize][crate_pos.0 as usize] = '.';
    }

    // None
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

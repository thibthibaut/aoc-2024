use std::{
    collections::{HashSet, VecDeque},
    ops::RangeBounds,
};

use glam::IVec2;
use itertools::Itertools;
use rayon::prelude::*;
advent_of_code::solution!(10);

// Define the four cardinal directions
const DIRECTIONS: [IVec2; 4] = [
    IVec2::new(0, -1), // North
    IVec2::new(0, 1),  // South
    IVec2::new(1, 0),  // East
    IVec2::new(-1, 0), // West
];

pub fn part_one(input: &str) -> Option<usize> {
    let map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let width = map[0].len() as i32;
    let height = map.len() as i32;

    // find all trailheads
    let trailheads = map
        .iter()
        .flatten()
        .positions(|x| *x == 0)
        .map(|pos| IVec2 {
            x: pos as i32 % width,
            y: pos as i32 / width,
        })
        .collect_vec();

    trailheads
        .into_par_iter()
        .map(|trailhead| {
            // Let's do a BFS for once
            let mut to_visit: VecDeque<IVec2> = VecDeque::new();
            let mut found_nines: HashSet<IVec2> = HashSet::new();
            to_visit.push_back(trailhead);

            while let Some(cursor) = to_visit.pop_front() {
                let map_value = map[cursor.y as usize][cursor.x as usize];
                if map_value == 9 && !found_nines.contains(&cursor) {
                    found_nines.insert(cursor);
                    continue;
                }
                for direction in DIRECTIONS {
                    let next_cursor = cursor + direction;
                    if (0..width).contains(&next_cursor.x) && (0..height).contains(&next_cursor.y) {
                        let new_map_value = map[next_cursor.y as usize][next_cursor.x as usize];
                        if new_map_value - map_value == 1 {
                            to_visit.push_back(next_cursor);
                        }
                    }
                }
            }
            found_nines.len()
        })
        .sum::<usize>()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let width = map[0].len() as i32;
    let height = map.len() as i32;

    // find all trailheads
    let trailheads = map
        .iter()
        .flatten()
        .positions(|x| *x == 0)
        .map(|pos| IVec2 {
            x: pos as i32 % width,
            y: pos as i32 / width,
        })
        .collect_vec();

    trailheads
        .into_par_iter()
        .map(|trailhead| {
            // Let's do a BFS for once
            let mut to_visit: VecDeque<IVec2> = VecDeque::new();
            let mut found_nines: Vec<IVec2> = Vec::new();
            to_visit.push_back(trailhead);

            while let Some(cursor) = to_visit.pop_front() {
                let map_value = map[cursor.y as usize][cursor.x as usize];
                if map_value == 9 {
                    found_nines.push(cursor);
                    continue;
                }
                for direction in DIRECTIONS {
                    let next_cursor = cursor + direction;
                    if (0..width).contains(&next_cursor.x) && (0..height).contains(&next_cursor.y) {
                        let new_map_value = map[next_cursor.y as usize][next_cursor.x as usize];
                        if new_map_value - map_value == 1 {
                            to_visit.push_back(next_cursor);
                        }
                    }
                }
            }
            found_nines.len()
        })
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}

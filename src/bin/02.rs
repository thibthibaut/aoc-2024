use std::{collections::HashMap, iter::zip};

use itertools::Itertools;

advent_of_code::solution!(2);

fn valid_line(entry: &Vec<u32>) -> bool {
    // compute the difference between two consecutive elements
    let diffs = entry
        .windows(2)
        .map(|win| {
            let [x, y] = win else { unreachable!() };
            *y as i32 - *x as i32
        })
        .collect_vec();

    // Check if they all have the same sign
    let signs = diffs.iter().map(|&x| x.signum()).collect_vec();

    // Check if all signs are the same
    let all_same_sign = signs.iter().all(|&s| s == signs[0]);

    // Get the minimum and max absolutee
    let abs = diffs.iter().map(|&x| x.abs());
    let min_abs = abs.clone().min().unwrap();
    let max_abs = abs.clone().max().unwrap();

    all_same_sign && min_abs > 0 && max_abs < 4
}

fn valid_line2(entry: &Vec<u32>) -> bool {
    if !valid_line(entry) {
        for i in 0..entry.len() {
            let mut entry_removed = entry.clone();
            entry_removed.remove(i);
            if valid_line(&entry_removed) {
                return true;
            }
        }
        return false;
    } else {
        true
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect_vec()
        })
        .filter(valid_line)
        .count() as u32;

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect_vec()
        })
        .filter(valid_line2)
        .count() as u32;

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}

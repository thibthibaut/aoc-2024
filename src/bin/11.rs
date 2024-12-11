use std::collections::HashMap;

use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let input = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();

    fn process_stone(stone: u64, depth: i32) -> u64 {
        if depth == 25 {
            return 1;
        }
        if stone == 0 {
            return process_stone(1, depth + 1);
        }
        let num_digits = stone.ilog10() + 1;
        if num_digits % 2 == 0 {
            let divisor = 10_u64.pow(num_digits / 2);
            let (left, right) = (stone / divisor, stone % divisor);
            return process_stone(left, depth + 1) + process_stone(right, depth + 1);
        }
        process_stone(stone * 2024, depth + 1)
    }

    input
        .into_par_iter()
        .map(|stone| process_stone(stone, 0))
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();

    fn process_stone(stone: u64, depth: i32, memo: &mut HashMap<(u64, i32), u64>) -> u64 {
        // For each stone, depth we'll store the result in a memo,
        // this way we avoid recomputing multiple times the same thing

        // Check if result is already memoized
        if let Some(&result) = memo.get(&(stone, depth)) {
            return result;
        }

        // When we reach the desired depth
        if depth == 75 {
            return 1;
        }

        if stone == 0 {
            let result = process_stone(1, depth + 1, memo);
            // remember this result
            memo.insert((stone, depth), result);
            return result;
        }

        // Calculate result based on conditions
        let result = {
            let num_digits = stone.ilog10() + 1;
            if num_digits % 2 == 0 {
                let divisor = 10_u64.pow(num_digits / 2);
                let (left, right) = (stone / divisor, stone % divisor);
                process_stone(left, depth + 1, memo) + process_stone(right, depth + 1, memo)
            } else {
                process_stone(stone * 2024, depth + 1, memo)
            }
        };

        // remember this result
        memo.insert((stone, depth), result);
        result
    }

    input
        .into_par_iter()
        .map(|stone| {
            let mut memo: HashMap<(u64, i32), u64> = HashMap::new();
            process_stone(stone, 0, &mut memo)
        })
        .sum::<u64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}

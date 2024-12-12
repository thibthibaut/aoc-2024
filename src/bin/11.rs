use itertools::Itertools;
use memoize::memoize;
use rayon::prelude::*;

advent_of_code::solution!(11);

fn solve(input: &str, max_depth: i32) -> Option<u64> {
    let input = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();

    #[memoize]
    fn process_stone(stone: u64, depth: i32, max: i32) -> u64 {
        if depth == max {
            return 1;
        }
        if stone == 0 {
            return process_stone(1, depth + 1, max);
        }
        let num_digits = stone.ilog10() + 1;
        if num_digits % 2 == 0 {
            let divisor = 10_u64.pow(num_digits / 2);
            let (left, right) = (stone / divisor, stone % divisor);
            return process_stone(left, depth + 1, max) + process_stone(right, depth + 1, max);
        }
        process_stone(stone * 2024, depth + 1, max)
    }

    input
        .into_par_iter()
        .map(|stone| process_stone(stone, 0, max_depth))
        .sum::<u64>()
        .into()
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 25)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 75)
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

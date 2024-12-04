use itertools::Itertools;
use std::iter::zip;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    for line in input.lines() {
        let split = line.trim().split(' ').collect_vec();
        let left_val = split.first().unwrap().parse::<u32>().unwrap();
        let right_val = split.last().unwrap().parse::<u32>().unwrap();
        left.push(left_val);
        right.push(right_val);
    }
    left.sort();
    right.sort();

    let mut sum: u32 = 0;
    for (l, r) in zip(left, right) {
        sum += r.abs_diff(l);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    for line in input.lines() {
        let split = line.trim().split(' ').collect_vec();
        let left_val = split.first().unwrap().parse::<u32>().unwrap();
        let right_val = split.last().unwrap().parse::<u32>().unwrap();
        left.push(left_val);
        right.push(right_val);
    }

    let mut sum: u32 = 0;
    for val in left {
        let count = right.iter().filter(|x| **x == val).count() as u32;
        sum += val * count;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

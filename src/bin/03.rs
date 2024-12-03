use itertools::Itertools;
use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let a: u32 = cap[1].parse().unwrap();
            let b: u32 = cap[2].parse().unwrap();
            a * b
        })
        .reduce(|a, b| a + b)
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let dos_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    // Get indices of dos and don'ts in the string
    let dos_idx = dos_re.find_iter(&input).map(|m| m.start()).collect_vec();
    let dont_idx = dont_re.find_iter(&input).map(|m| m.start()).collect_vec();

    // Create intervals where the muls are "disabled" by looking at the ranges
    // of dont's <--> do
    let mut disabled_intervals = Vec::new();
    for dont in &dont_idx {
        // Find the next do that is greater than the current dont
        if let Some(next_do) = dos_idx.iter().find(|&do_| do_ > &dont) {
            disabled_intervals.push(*dont..*next_do);
        } else {
            disabled_intervals.push(*dont..usize::MAX);
        }
    }

    // Same capture as part 1 but this time we check if we are inside a
    // disabled range
    re.captures_iter(input)
        .filter_map(|cap| {
            // Get index of `mul`
            let start = cap.get(0).unwrap().start();
            // Check if it's in a forbidden inteval
            for interval in &disabled_intervals {
                if interval.contains(&start) {
                    return None;
                }
            }
            let a: u32 = cap[1].parse().unwrap();
            let b: u32 = cap[2].parse().unwrap();
            Some(a * b)
        })
        .reduce(|a, b| a + b)
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}

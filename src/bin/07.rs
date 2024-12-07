use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rayon::prelude::*;
advent_of_code::solution!(7);

#[derive(Debug)]
struct Equation {
    target: u64,
    numbers: Vec<u64>,
}

// Parse the input
// Parses a single u64 from a string slice
fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}
// Parses the list of numbers (eg. 32 41 55)
fn parse_number_list(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, parse_u64)(input)
}
// Parses an equation 123: 11 12 13
fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (parsed, (target, numbers)) =
        separated_pair(parse_u64, tag(": "), parse_number_list)(input)?;
    Ok((parsed, Equation { target, numbers }))
}
// Parse the whole input by splitting line_endings
fn parse_input(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(line_ending, parse_equation)(input)
}

// Reduces the number slice by tring recursivly to merge the
// two first entry into a sum or a product
fn recursive_fold(numbers: &[u64], target: u64) -> bool {
    // Get the first two numbers
    let a = numbers[0];
    let b = numbers[1];

    // Generate the two possiblities
    let sum = a + b;
    let prod = a * b;

    // Stop if the slice has only two numbers
    if numbers.len() == 2 {
        // If we equal target we can return true
        if sum == target || prod == target {
            return true;
        }
        return false;
    }
    // Create a new slice with the fold and call recusivly
    // Sum
    let mut new_numbers = numbers[1..].to_owned();
    new_numbers[0] = sum;
    let result_sum = recursive_fold(&new_numbers, target);
    // Prod
    let mut new_numbers = numbers[1..].to_owned();
    new_numbers[0] = prod;
    let result_prod = recursive_fold(&new_numbers, target);

    // Return true if one them is true
    result_sum || result_prod
}

fn recursive_fold2(numbers: &[u64], target: u64) -> bool {
    // Get the first two numbers
    let a = numbers[0];
    let b = numbers[1];

    // Generate the three possiblities
    let sum = a + b;
    let prod = a * b;
    let cat = a * 10u64.pow(b.ilog10() + 1) + b;

    // Stop if the slice has only two numbers
    if numbers.len() == 2 {
        if [sum, prod, cat].iter().any(|x| *x == target) {
            return true;
        }
        return false;
    }

    [sum, prod, cat]
        .iter()
        .map(|op| {
            let mut new_numbers = numbers[1..].to_owned();
            new_numbers[0] = *op;
            return recursive_fold2(&new_numbers, target);
        })
        .any(|x| x)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, input) = parse_input(input).expect("Failed to parse input");

    input
        .iter()
        .filter_map(|eq| {
            let out = recursive_fold(&eq.numbers, eq.target);
            if out {
                return Some(eq.target);
            }
            None
        })
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, input) = parse_input(input).expect("Failed to parse input");

    input
        .par_iter()
        .filter_map(|eq| {
            let out = recursive_fold2(&eq.numbers, eq.target);
            if out {
                return Some(eq.target);
            }
            None
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}

use std::{collections::VecDeque, iter::zip};

use itertools::Itertools;
use std::cmp::Ordering::{Equal, Greater, Less};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let numbers: Vec<i32> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|x| x.to_string().parse::<i32>().unwrap())
        .collect();

    let mut files: VecDeque<(usize, i32)> = numbers
        .iter()
        .step_by(2)
        .enumerate()
        .map(|(i, &x)| (i, x))
        .collect();

    let mut spaces: VecDeque<i32> = numbers.iter().skip(1).step_by(2).copied().collect();
    let mut output: Vec<(usize, i32)> = Vec::new();
    // Push the first file into the output
    output.push(files.pop_front().unwrap());

    while !spaces.is_empty() {
        if files.is_empty() {
            break;
        }
        // Take the last file
        let (id, file) = *files.back().unwrap();
        // Take the first space
        let space = *spaces.front().unwrap();

        match file.cmp(&space) {
            Greater => {
                // All the space is going to be filled
                spaces.pop_front();
                // Update the file with the difference
                *files.back_mut().unwrap() = (id, file - space);
                // space-many files have been added the the answer
                output.push((id, space));
                // because we move to next space, we need to push the file
                output.push(files.pop_front().unwrap());
            }
            Less => {
                // The file will be gone
                files.pop_back();
                // Update the space with the difference
                *spaces.front_mut().unwrap() = space - file;
                output.push((id, file));
            }
            Equal => {
                // All files will go in the space, pop both
                spaces.pop_front();
                files.pop_back();
                output.push((id, file));
                output.push(files.pop_front().unwrap());
            }
        }
    }

    let mut result = 0;
    let mut idx = 0;
    for (id, num) in &output {
        // Using arithmetic sequence sum formula
        let n = *num as usize;
        // Skip calculation if n is 0
        if n == 0 {
            continue;
        }

        let first_term = idx;
        let last_term = idx + (n - 1);
        let sequence_sum = n * (first_term + last_term) / 2;

        result += id * sequence_sum;
        idx += n;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let line = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|x| x.to_string().parse::<i32>().unwrap());

    let mut files: VecDeque<(usize, i32)> = line.clone().step_by(2).enumerate().collect();
    let mut spaces: VecDeque<i32> = line.skip(1).step_by(2).collect();

    return None;
    dbg!(&files, &spaces);

    let mut output: Vec<(usize, i32)> = Vec::new();
    // Push the first file into the output
    output.push(files.pop_front().unwrap());

    // // Remove zero-spaces
    // let mut spaces: VecDeque<i32> = spaces.into_iter().filter(|x| *x != 0).collect();
    // let mut files: VecDeque<(usize, i32)> = files.into_iter().filter(|x| x.1 != 0).collect();
    // Move the files into the spaces, until there's no space left on device :P
    loop {
        if files.is_empty() || spaces.is_empty() {
            break;
        }
        // Test
        // Take the last file
        for (id, mut file) in files.iter().rev() {
            // Try all the spaces to put it in
            let mut has_break = false;
            for space in spaces.iter_mut() {
                if space > &mut file {
                    // The file with go there
                    // Update the space accordingly
                    *space -= file;
                    if *space < 0 {
                        panic!("cannot happen");
                    }
                    if *space == 0 {
                        spaces.pop_front();
                    }

                    has_break = true;
                    break;
                }
            }
        }
    }
    None

    // let mut result = 0;
    // let mut idx = 0;
    // for (id, num) in &output {
    //     // Using arithmetic sequence sum formula
    //     let n = *num as usize;
    //     let first_term = idx;
    //     let last_term = idx + (n - 1);
    //     let sequence_sum = n * (first_term + last_term) / 2;

    //     result += id * sequence_sum;
    //     idx += n;
    // }
    // Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_one_many_spaces() {
        let input = "191919";
        /*
        19191
        0.........1.........2
        012
        1*2 + 2*1 = 4
        */
        let result = part_one(input);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_spaces_size_zero() {
        let input = "1010101";
        /*
        1010101
        0123
        1*1 + 2*2 + 3*3 = 14
        */
        let result = part_one(input);
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

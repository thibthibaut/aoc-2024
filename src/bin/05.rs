use itertools::Itertools;
use nom::{
    character::complete::{char, digit1, line_ending},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(5);

#[derive(Debug)]
struct Data {
    rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

// Parses a single u32 from a string slice
fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

// Parses a pair of u32 values separated by '|'
fn parse_pair(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(parse_u32, char('|'), parse_u32)(input)
}

// Parses a list of pairs separated by newlines
fn parse_pairs(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(line_ending, parse_pair)(input)
}

// Parses a sequence of u32 values separated by ','
fn parse_sequence(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(char(','), parse_u32)(input)
}

// Parses the entire data structure
fn parse_data(input: &str) -> IResult<&str, Data> {
    let (input, rules) = terminated(parse_pairs, many1(line_ending))(input)?;
    let (input, updates) = separated_list1(line_ending, parse_sequence)(input)?;
    Ok((input, Data { rules, updates }))
}

/// Function to reduce the ordering rules to a hiererchy chain A < B < C < D
fn reduce_to_chain(rules: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new(); // Graph structure, for each node contains all its connexions
    let mut in_degree: HashMap<u32, u32> = HashMap::new(); // Counts how many inwards connexion for each node
    let mut nodes: HashSet<u32> = HashSet::new(); // The list of nodes

    // Build the graph and track in-degrees
    for &(a, b) in rules {
        graph.entry(a).or_default().push(b);
        *in_degree.entry(b).or_default() += 1; // b just got a new inward connexion
        in_degree.entry(a).or_default();
        nodes.insert(a);
        nodes.insert(b);
    }

    // Find all nodes with zero in-degree to start
    let mut queue: VecDeque<u32> = VecDeque::new();
    for node in nodes.iter() {
        if *in_degree.get(node).expect("Didn't find the node !") == 0 {
            queue.push_back(*node);
        }
    }
    // Perform topological sort
    let mut result = Vec::new();
    while let Some(node) = queue.pop_front() {
        result.push(node);
        if let Some(neighbors) = graph.get(&node) {
            // Reduce the in-degree of each neighbor by 1 because one of its dependencies
            // (the current node) has now been processed.
            for &neighbor in neighbors {
                let degree = in_degree.get_mut(&neighbor).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    // if the degree of the neighbor is zero, it means it has no more
                    // inward conexion, so we'll deal with it next time !
                    queue.push_back(neighbor);
                }
            }
        }
    }
    if result.len() != nodes.len() {
        panic!("The input rules contain a cycle and cannot be reduced to a chain.");
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, data) = parse_data(input).expect("Failed to parse");
    data.updates
        .par_iter()
        .filter(|update| {
            // filter the valid updates by looking at all the rules
            data.rules.iter().all(|rule| {
                let first = update.iter().position(|x| *x == rule.0);
                let second = update.iter().position(|x| *x == rule.1);
                match (first, second) {
                    (Some(a), Some(b)) => a < b, // valid only if first < second
                    _ => true,                   // also valid if one of the two is not in the rule
                }
            })
        })
        .map(|update| update[update.len() / 2]) // take the middle number
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, data) = parse_data(input).expect("Failed to parse");

    data.updates
        .par_iter()
        .filter_map(|update| {
            let rules = data
                .rules
                .clone()
                .into_iter()
                .filter(|rule| update.contains(&rule.0) && update.contains(&rule.1))
                .collect_vec();
            // Reduce the graph rules to an hierarchy list
            let chain = reduce_to_chain(&rules);
            // Sort the update using the chain
            let mut new_update = update.clone();
            new_update.sort_by(|a, b| {
                let a_pos = chain
                    .iter()
                    .position(|r| r == a)
                    .expect("Expected to find a in the chain rule");
                let b_pos = chain
                    .iter()
                    .position(|r| r == b)
                    .expect("Expected to find b in the chain rule");
                // Return the compartar of the positions in the chain rule
                a_pos.cmp(&b_pos)
            });
            if *update != new_update {
                Some(new_update[new_update.len() / 2])
            } else {
                None
            }
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}

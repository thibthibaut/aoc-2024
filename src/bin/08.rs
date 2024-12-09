use std::collections::HashMap;

use glam::IVec2;
use itertools::Itertools;

advent_of_code::solution!(8);

fn in_bounds(pos: &IVec2, width: usize, height: usize) -> bool {
    pos.x >= 0 && pos.y >= 0 && pos.x < width as i32 && pos.y < height as i32
}
pub fn part_one(input: &str) -> Option<usize> {
    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let width = map[0].len();
    let height = map.len();

    // Find the location of the frequencies
    let frequencies = map
        .iter()
        .flatten()
        .enumerate()
        .filter_map(|(pos, c)| match c {
            '.' => None,
            f => Some((
                *f,
                IVec2 {
                    x: (pos % width) as i32,
                    y: (pos / width) as i32,
                },
            )),
        })
        .collect_vec();

    // Convert to a Hashmap where the key is the frequency
    // and the value is a vec containing all the positions
    let mut frequencies_map: HashMap<char, Vec<IVec2>> = HashMap::new();
    for (f, pos) in &frequencies {
        frequencies_map.entry(*f).or_default().push(*pos);
    }

    frequencies_map
        .iter()
        .filter_map(|(_, positions)| {
            match positions.len() {
                1 => None, // Ignore if there is only 1 frequency
                _ => {
                    Some(
                        // Otherwise find the antinodes by looking at all arrangements of 2
                        positions
                            .iter()
                            .combinations(2)
                            .map(|combi| {
                                let point_a = combi[0];
                                let point_b = combi[1];
                                // Compute the vector between a and b
                                let vector = point_b - point_a;
                                // Apply this vector to both end, in oppsite directions for a
                                vec![point_a - vector, point_b + vector]
                            })
                            .collect_vec(),
                    )
                }
            }
        })
        // Flatten evenrything to have just a list of antinodes
        .flatten()
        .flatten()
        // remove duplicates and filter the antinodes outside the map
        .unique()
        // Keep only those which are inside the map
        .filter(|antinode| in_bounds(antinode, width, height))
        .count()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let width = map[0].len();
    let height = map.len();
    // Find the location of the frequencies
    let frequencies = map
        .iter()
        .flatten()
        .enumerate()
        .filter_map(|(pos, c)| match c {
            '.' => None,
            f => Some((
                *f,
                IVec2 {
                    x: (pos % width) as i32,
                    y: (pos / width) as i32,
                },
            )),
        })
        .collect_vec();

    // Convert to a Hashmap where the key is the frequency
    // and the value is a vec containing all the positions
    let mut frequencies_map: HashMap<char, Vec<IVec2>> = HashMap::new();
    for (f, pos) in &frequencies {
        frequencies_map.entry(*f).or_default().push(*pos);
    }

    frequencies_map
        .iter()
        .filter_map(|(_, positions)| {
            match positions.len() {
                1 => None, // Ignore if there is only 1 frequency
                _ => {
                    Some(
                        // Otherwise find the antinodes by looking at all arrangements of 2
                        positions
                            .iter()
                            .combinations(2)
                            .map(|combi| {
                                let point_a = combi[0];
                                let point_b = combi[1];
                                // Compute the vector between a and b
                                let vector = point_b - point_a;
                                // Apply this vector as many times as needed
                                // to both end, in oppsite directions for a
                                let mut antinodes = Vec::new();
                                let mut new_point = point_a - vector;
                                while in_bounds(&new_point, width, height) {
                                    antinodes.push(new_point);
                                    new_point -= vector;
                                }
                                let mut new_point = point_b + vector;
                                while in_bounds(&new_point, width, height) {
                                    antinodes.push(new_point);
                                    new_point += vector;
                                }
                                antinodes.push(*point_a);
                                antinodes.push(*point_b);
                                antinodes
                            })
                            .collect_vec(),
                    )
                }
            }
        })
        // Flatten evenrything to have just a list of antinodes
        .flatten()
        .flatten()
        // remove duplicates and filter the antinodes outside the map
        .unique()
        // Keep only those which are inside the map
        .filter(|antinode| in_bounds(antinode, width, height))
        .count()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}

use itertools::Itertools;
advent_of_code::solution!(4);

#[derive(Debug)]
struct Position {
    pub x: i32,
    pub y: i32,
}
impl Position {
    /// Check if the current position is inside the bounds
    fn in_bounds(&self, map: &[Vec<char>]) -> bool {
        let width = map[0].len() as i32;
        let height = map.len() as i32;
        self.x >= 0 && self.x < width && self.y >= 0 && self.y < height
    }

    fn apply_direction(&self, dir: (i32, i32)) -> Self {
        Position {
            x: self.x + dir.0,
            y: self.y + dir.1,
        }
    }
}
fn get_all_directions() -> [(i32, i32); 8] {
    [
        (-1, -1), // top-left
        (0, -1),  // top
        (1, -1),  // top-right
        (-1, 0),  // left
        (1, 0),   // right
        (-1, 1),  // bottom-left
        (0, 1),   // bottom
        (1, 1),   // bottom-right
    ]
}

/// Recursive function to explore the map in a given direction
/// if no direction is given, explore all 8 possible directions
fn explore(map: &[Vec<char>], pos: Position, dir: Option<(i32, i32)>, letters: &str) -> u32 {
    let target = String::from("XMAS");
    let mut found_count = 0; // count how many we found

    // Exit if pos is out of bounds
    if !pos.in_bounds(map) {
        return 0;
    }

    // Get the current char and append it to letters
    let current_char = map[pos.y as usize][pos.x as usize];
    let mut new_letters = letters.to_owned();
    new_letters.push(current_char);

    // Exit if XMAS doesn't contain letters
    if !target.starts_with(&new_letters) {
        return 0;
    }
    // Exit if letters == XMAS
    if new_letters == target {
        return 1;
    }

    // Explore all directions
    if new_letters.len() == 1 {
        for direction in get_all_directions() {
            let new_pos = pos.apply_direction(direction);
            found_count += explore(map, new_pos, Some(direction), &new_letters);
        }
    } else {
        // Explore the current direction
        let new_pos = pos.apply_direction(dir.expect("Direction must be defined"));
        found_count += explore(map, new_pos, dir, &new_letters);
    }
    found_count
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut count = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            let pos = Position {
                x: col as i32,
                y: row as i32,
            };
            let letters = String::new();
            count += explore(&map, pos, None, &letters);
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    // These are all the possible patterns for X-MAS
    let patterns = [
        [['M', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'S']],
        [['S', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'M']],
        [['S', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'M']],
        [['M', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'S']],
    ];

    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    // We run convolution over the map using the patterns as the convolution kernel
    let mut count = 0;
    for row in 1..map.len() - 1 {
        for col in 1..map[0].len() - 1 {
            for pattern in &patterns {
                let mut valid = 0;
                for krow in 0..3 {
                    for kcol in 0..3 {
                        let val_map = map[row + krow - 1][col + kcol - 1];
                        let val_kernel = pattern[krow][kcol];
                        if val_map == val_kernel {
                            valid += 1
                        }
                    }
                } // end for kernel
                if valid == 5 {
                    count += 1;
                    break;
                }
            } // end for patterns
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}

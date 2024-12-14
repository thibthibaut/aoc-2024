use glam::IVec2;
use itertools::{iproduct, Itertools};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{anychar, digit1, line_ending, multispace0, multispace1, one_of, space1},
    combinator::{map, map_res},
    complete::take,
    multi::{many_till, separated_list1},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

advent_of_code::solution!(13);

#[derive(Debug)]
struct Game {
    button_a: IVec2,
    button_b: IVec2,
    prize: IVec2,
}

// Parse the input
// Parses a single i32 from a string slice
fn parse_i32(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}
fn parse_coord_component(input: &str) -> IResult<&str, i32> {
    preceded(tuple((anychar, anychar)), parse_i32)(input)
}
fn parse_coords(input: &str) -> IResult<&str, IVec2> {
    let (parsed, (x, y)) =
        separated_pair(parse_coord_component, tag(", "), parse_coord_component)(input)?;
    Ok((parsed, IVec2 { x, y }))
}
fn parse_line(input: &str) -> IResult<&str, IVec2> {
    preceded(tuple((take_until(": "), tag(": "))), parse_coords)(input)
}
fn parse_game(input: &str) -> IResult<&str, Game> {
    map(
        tuple((
            parse_line,
            multispace1,
            parse_line,
            multispace1,
            parse_line,
            multispace0,
        )),
        |(button_a, _, button_b, _, prize, _)| Game {
            button_a,
            button_b,
            prize,
        },
    )(input)
}
fn parse_input(input: &str) -> IResult<&str, Vec<Game>> {
    let mut games = Vec::new();
    let mut remaining = input;

    while !remaining.is_empty() {
        let (rest, game) = parse_game(remaining)?;
        games.push(game);
        remaining = rest.trim();
    }
    Ok((remaining, games))
}

pub fn part_one(input: &str) -> Option<i32> {
    let (_, games) = parse_input(input).expect("Failed to parse input");

    games
        .iter()
        .filter_map(|game| {
            // Compute maximum number of times we need to push the buttons
            let max_a_press = 101; //(game.prize / game.button_a).max_element();
            let max_b_press = 101; //(game.prize / game.button_b).max_element();

            let costs = iproduct!(1..max_a_press, 1..max_b_press)
                .filter_map(|(a_press, b_press)| {
                    let cost = 3 * a_press + b_press;
                    let pos = a_press * game.button_a + b_press * game.button_b;
                    if pos == game.prize {
                        return Some(cost);
                    }
                    None
                })
                .collect_vec();

            costs.into_iter().min()
        })
        .sum::<i32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}

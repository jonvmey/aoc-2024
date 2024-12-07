advent_of_code::solution!(1);
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn parse_line(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(complete::i32, tag("   "), complete::i32)(input)
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (_, lines) = separated_list0(newline, parse_line)(input).expect("parsing shouldn't fail");

    let left: Vec<i32> = lines.iter().map(|(e, _)| *e).collect();
    let right: Vec<i32> = lines.iter().map(|(_, e)| *e).collect();

    (left, right)
}

pub fn part_one(input: &str) -> Option<i32> {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();

    Some(
        left.into_iter()
            .zip(right)
            .map(|(l, r)| (l - r).abs())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let (left, right) = parse_input(input);
    let counts = right.into_iter().counts();

    Some(
        left.into_iter()
            .map(|e| e * (*counts.get(&e).unwrap_or(&0) as i32))
            .sum(),
    )
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

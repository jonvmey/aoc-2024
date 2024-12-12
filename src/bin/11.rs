use std::collections::HashMap;

use nom::{bytes::complete::tag, character, multi::separated_list1, IResult};

advent_of_code::solution!(11);

fn parse_stones(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(" "), character::complete::u64)(input)
}

fn parse_input(input: &str) -> HashMap<u64, u64> {
    let (_, stones) = parse_stones(input).expect("should parse");
    let mut map = HashMap::new();

    for stone in stones {
        *map.entry(stone).or_insert(0) += 1;
    }

    map
}

fn num_decimal_digits(n: u64) -> u32 {
    n.to_string().len() as u32
}

fn apply_blink_rules(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        return (1, None);
    }

    let num_digits = num_decimal_digits(stone);
    if num_digits % 2 == 0 {
        let divisor = 10u64.pow(num_digits / 2);

        return (stone / divisor, Some(stone % divisor));
    }

    (stone * 2024, None)
}

fn evaluate_blink(old_stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::new();

    for (stone, count) in old_stones {
        let (left, right) = apply_blink_rules(stone);

        *new_stones.entry(left).or_insert(0) += count;
        if let Some(right) = right {
            *new_stones.entry(right).or_insert(0) += count;
        }
    }

    new_stones
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones = parse_input(input);

    for _ in 0..25 {
        stones = evaluate_blink(stones);
    }

    Some(stones.values().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut stones = parse_input(input);

    for _ in 0..75 {
        stones = evaluate_blink(stones);
    }

    Some(stones.values().sum())
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
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}

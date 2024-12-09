use std::cmp::Ordering;

use nom::{
    bytes::complete::tag,
    character::{self, complete::newline},
    multi::{many0, separated_list0},
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(5);

type OrderingRule = (u32, u32);
type UpdatePages = Vec<u32>;

fn parse_ordering_rule(input: &str) -> IResult<&str, OrderingRule> {
    separated_pair(character::complete::u32, tag("|"), character::complete::u32)(input)
}

fn strip_newlines(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(newline)(input)?;
    Ok((input, ()))
}

fn parse_update(input: &str) -> IResult<&str, UpdatePages> {
    let (input, update) = separated_list0(tag(","), character::complete::u32)(input)?;
    let (input, _) = newline(input)?;

    Ok((input, update))
}

fn parse_updates(input: &str) -> IResult<&str, Vec<UpdatePages>> {
    many0(parse_update)(input)
}
fn parse_input(input: &str) -> (Vec<OrderingRule>, Vec<UpdatePages>) {
    let (input, rules) =
        separated_list0(newline, parse_ordering_rule)(input).expect("rules should parse");
    let (input, _) = strip_newlines(input).expect("shouldn't fail");
    let (_, updates) = parse_updates(input).expect("updates should parse");

    (rules, updates)
}

fn cmp_pages(lhs: u32, rhs: u32, rules: &[OrderingRule]) -> Ordering {
    for (before, after) in rules {
        if *before == lhs && *after == rhs {
            return Ordering::Less;
        }
        if *before == rhs && *after == lhs {
            return Ordering::Greater;
        }
    }

    Ordering::Equal
}

fn correctly_ordered(pages: &[u32], rules: &[OrderingRule]) -> bool {
    pages.is_sorted_by(|lhs, rhs| cmp_pages(*lhs, *rhs, rules) != Ordering::Greater)
}

fn correct_ordering(mut pages: Vec<u32>, rules: &[OrderingRule]) -> Vec<u32> {
    pages.sort_by(|lhs, rhs| cmp_pages(*lhs, *rhs, rules));

    pages
}

fn middle_page(pages: &[u32]) -> Option<u32> {
    if !pages.is_empty() {
        return Some(pages[pages.len() / 2]);
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);

    Some(
        updates
            .iter()
            .filter(|update| correctly_ordered(update, &rules))
            .map(|update| middle_page(update).unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);
    let mut sum = 0;

    for update in updates {
        if correctly_ordered(&update, &rules) {
            continue;
        }
        let corrected = correct_ordering(update.clone(), &rules);
        sum += middle_page(&corrected).expect("should be a middle page");
    }

    Some(sum)
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

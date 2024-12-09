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

fn correctly_ordered(pages: &[u32], rules: &[OrderingRule]) -> bool {
    rules
        .iter()
        .filter_map(|(before, after)| {
            let pos_before = pages.iter().position(|e| e == before);
            let pos_after = pages.iter().position(|e| e == after);

            match (pos_before, pos_after) {
                (Some(pos_before), Some(pos_after)) => Some(pos_before < pos_after),
                _ => None,
            }
        })
        .all(|b| b)
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

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}

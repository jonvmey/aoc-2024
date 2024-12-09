use nom::{
    bytes::complete::tag,
    character::{self, complete::newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(7);

struct Equation {
    result: i64,
    operands: Vec<i64>,
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (input, (result, operands)) = separated_pair(
        character::complete::i64,
        tag(": "),
        separated_list1(tag(" "), character::complete::i64),
    )(input)?;

    Ok((input, Equation { result, operands }))
}

fn parse_input(input: &str) -> Vec<Equation> {
    let (_, equations) =
        separated_list1(newline, parse_equation)(input).expect("input should parse");

    equations
}

fn valid_equation(result: i64, operands: &[i64]) -> bool {
    match operands.len() {
        0 => false,
        1 => result == *operands.first().expect("can't be none"),
        _ => {
            let (operand, rest) = operands.split_last().expect("can't be none");

            valid_equation(result - operand, rest)
                || (result % operand == 0 && valid_equation(result / operand, rest))
        }
    }
}

fn valid_equation2(result: i64, operands: &[i64]) -> bool {
    if result < 0 {
        return false;
    }

    match operands.len() {
        0 => false,
        1 => result == *operands.first().expect("can't be none"),
        _ => {
            let (operand, rest) = operands.split_last().expect("can't be none");

            if valid_equation2(result - operand, rest)
                || (result % operand == 0 && valid_equation2(result / operand, rest))
            {
                true
            } else {
                let res_str = result.to_string();
                let op_str = operand.to_string();

                if let Some(new_result) = res_str.strip_suffix(&op_str) {
                    if new_result.is_empty() {
                        false
                    } else {
                        valid_equation2(new_result.parse::<i64>().expect("should parse"), rest)
                    }
                } else {
                    false
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let equations = parse_input(input);

    Some(
        equations
            .into_iter()
            .filter_map(|equation| {
                if valid_equation(equation.result, &equation.operands) {
                    Some(equation.result)
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let equations = parse_input(input);

    Some(
        equations
            .into_iter()
            .filter_map(|equation| {
                if valid_equation2(equation.result, &equation.operands) {
                    Some(equation.result)
                } else {
                    None
                }
            })
            .sum(),
    )
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

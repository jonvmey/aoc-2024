use nom::{
    branch::alt,
    bytes::complete::tag,
    character,
    multi::{many0, many_till},
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(3);

enum Instruction {
    Mul { left: u32, right: u32 },
    Do,
    Dont,
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul(")(input)?;
    let (input, (left, right)) =
        separated_pair(character::complete::u32, tag(","), character::complete::u32)(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, Instruction::Mul { left, right }))
}

fn parse_do(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("do()")(input)?;

    Ok((input, Instruction::Do {}))
}

fn parse_dont(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("don't()")(input)?;

    Ok((input, Instruction::Dont {}))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, instruction)) = many_till(
        character::complete::anychar,
        alt((parse_mul, parse_do, parse_dont)),
    )(input)?;

    Ok((input, instruction))
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let (_, instructions) = many0(parse_instruction)(input).expect("parsing shouldn't fail");

    instructions
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = parse_input(input);

    Some(
        instructions
            .into_iter()
            .filter_map(|i| match i {
                Instruction::Mul { left, right } => Some(left * right),
                _ => None,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions = parse_input(input);
    let mut enabled = true;
    let mut sum = 0;

    for i in instructions {
        match i {
            Instruction::Mul { left, right } => {
                if enabled {
                    sum += left * right;
                }
            }
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}

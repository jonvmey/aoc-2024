use nom::{
    bytes::complete::tag,
    character::complete::{anychar, digit1, newline},
    multi::separated_list1,
    IResult,
};
use num::integer::lcm;

advent_of_code::solution!(13);

#[derive(Debug)]
struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

fn parse_button(input: &str) -> IResult<&str, (i64, i64)> {
    let (input, _) = tag("Button ")(input)?;
    let (input, _) = anychar(input)?; // A or B
    let (input, _) = tag(": X+")(input)?;
    let (input, x) = digit1(input)?;
    let (input, _) = tag(", Y+")(input)?;
    let (input, y) = digit1(input)?;
    let (input, _) = newline(input)?;

    Ok((input, (x.parse().unwrap(), y.parse().unwrap())))
}

fn parse_prize_location(input: &str) -> IResult<&str, (i64, i64)> {
    let (input, _) = tag("Prize: X=")(input)?;
    let (input, x) = digit1(input)?;
    let (input, _) = tag(", Y=")(input)?;
    let (input, y) = digit1(input)?;
    let (input, _) = newline(input)?;

    Ok((input, (x.parse().unwrap(), y.parse().unwrap())))
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, (a_x, a_y)) = parse_button(input)?;
    let (input, (b_x, b_y)) = parse_button(input)?;
    let (input, (prize_x, prize_y)) = parse_prize_location(input)?;

    Ok((
        input,
        Machine {
            a_x,
            a_y,
            b_x,
            b_y,
            prize_x,
            prize_y,
        },
    ))
}

fn parse_machines(input: &str) -> Vec<Machine> {
    let (_, machines) = separated_list1(newline, parse_machine)(input).unwrap();

    machines
}

fn prize_min_presses(machine: &Machine) -> Option<i64> {
    // Solve for B
    let lcm = lcm(machine.a_x, machine.a_y);
    let x_factor = lcm / machine.a_x;
    let y_factor = lcm / machine.a_y;

    let b_num = y_factor * machine.prize_y - x_factor * machine.prize_x;
    let b_den = y_factor * machine.b_y - x_factor * machine.b_x;

    if b_num % b_den != 0 {
        return None; // Non-integral solution
    }

    let b = b_num / b_den;

    if b < 0 {
        return None; // Negative solution
    }

    // Solve for A
    let a_num = machine.prize_x - machine.b_x * b;
    let a_den = machine.a_x;

    if a_num % a_den != 0 {
        return None; // Non-integral solution
    }

    let a = a_num / a_den;

    if a < 0 {
        return None; // Negative solution
    }

    Some(a * 3 + b)
}

pub fn part_one(input: &str) -> Option<i64> {
    let machines = parse_machines(input);

    Some(
        machines
            .into_iter()
            .filter_map(|machine| prize_min_presses(&machine))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut machines = parse_machines(input);

    machines.iter_mut().for_each(|machine| {
        machine.prize_x += 10_000_000_000_000;
        machine.prize_y += 10_000_000_000_000;
    });

    Some(
        machines
            .into_iter()
            .filter_map(|machine| prize_min_presses(&machine))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}

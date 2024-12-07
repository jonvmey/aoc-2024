use nom::{
    bytes::complete::tag,
    character::{self, complete::newline},
    multi::{many0, separated_list0},
    sequence::terminated,
    IResult,
};

advent_of_code::solution!(2);

fn parse_report(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list0(tag(" "), character::complete::i32)(input)
}

fn parse_reports(input: &str) -> Vec<Vec<i32>> {
    let (_, reports) =
        many0(terminated(parse_report, newline))(input).expect("parsing shouldn't fail");

    reports
}

fn is_safe(report: &[i32]) -> bool {
    assert!(report.len() >= 2);

    let mut safe_inc = true;
    let mut safe_dec = true;

    report
        .windows(2)
        .map(|window| window[1] - window[0])
        .for_each(|diff| {
            if !(1..=3).contains(&diff) {
                safe_inc = false;
            }
            if !(-3..=-1).contains(&diff) {
                safe_dec = false;
            }
        });

    safe_inc || safe_dec
}

fn is_dampener_safe(report: &[i32]) -> bool {
    assert!(report.len() >= 2);

    for i in 0..report.len() {
        let mut copy: Vec<i32> = report.to_vec();
        copy.remove(i);

        if is_safe(&copy) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<usize> {
    let reports = parse_reports(input);

    Some(
        reports
            .iter()
            .map(|report| is_safe(report))
            .filter(|b| *b)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let reports = parse_reports(input);

    Some(
        reports
            .iter()
            .map(|report| is_dampener_safe(report))
            .filter(|b| *b)
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}

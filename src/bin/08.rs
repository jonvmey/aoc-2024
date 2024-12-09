use std::collections::HashSet;

use grid_2d::{Coord, Grid, Size};
use num::integer::gcd;

advent_of_code::solution!(8);

fn parse_grid(input: &str) -> Grid<char> {
    let width = input
        .chars()
        .position(|c| c == '\n')
        .expect("input should contain more than 1 line") as u32;
    let height = input.lines().count() as u32;

    Grid::new_iterator(
        Size::new(width, height),
        input.chars().filter(|c| *c != '\n'),
    )
}

fn antenna_locations(map: &Grid<char>) -> Vec<(Coord, char)> {
    map.coord_iter()
        .filter_map(|pos| {
            let c = map.get(pos).expect("has to exist");
            if *c == '.' {
                None
            } else {
                Some((pos, *c))
            }
        })
        .collect()
}

fn points_on_line(p1: Coord, p2: Coord, dimensions: Size) -> Vec<Coord> {
    let distance = p2 - p1;
    let step = distance / gcd(distance.x.abs(), distance.y.abs());
    let mut points = vec![p1, p2];

    // Find points in one direction
    let mut p = p1;
    loop {
        p += step;
        if p.is_valid(dimensions) {
            points.push(p);
        } else {
            break;
        }
    }

    // Find points in the other direction
    let mut p = p1;
    loop {
        p -= step;
        if p.is_valid(dimensions) {
            points.push(p);
        } else {
            break;
        }
    }

    points
}

fn is_distance_multiple(d1: Coord, d2: Coord, multiplier: i32) -> bool {
    d1 * multiplier == d2
        || d1 * -multiplier == d2
        || d2 * multiplier == d1
        || d2 * -multiplier == d1
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_grid(input);
    let antennas = antenna_locations(&map);
    let mut antinodes = HashSet::<Coord>::new();

    for (index, (p1, first)) in antennas.iter().enumerate() {
        for (p2, second) in &antennas[index + 1..] {
            if first == second {
                let points = points_on_line(*p1, *p2, map.size());

                for point in points {
                    let d1 = p1 - point;
                    let d2 = p2 - point;
                    if is_distance_multiple(d1, d2, 2) {
                        antinodes.insert(point);
                    }
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_grid(input);
    let antennas = antenna_locations(&map);
    let mut antinodes = HashSet::<Coord>::new();

    for (index, (p1, first)) in antennas.iter().enumerate() {
        for (p2, second) in &antennas[index + 1..] {
            if first == second {
                points_on_line(*p1, *p2, map.size())
                    .into_iter()
                    .for_each(|p| {
                        antinodes.insert(p);
                    });
            }
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}

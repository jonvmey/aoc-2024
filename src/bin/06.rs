use std::collections::HashSet;

use grid_2d::{Coord, Grid, Size};
use rayon::prelude::*;

advent_of_code::solution!(6);

#[derive(Debug, Copy, Clone)]
enum Cell {
    Empty,
    Blocked,
}

impl From<char> for Cell {
    fn from(item: char) -> Self {
        match item {
            '.' | '^' => Cell::Empty,
            '#' => Cell::Blocked,
            _ => panic!("unexpected cell character"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    West,
    East,
    South,
}

fn parse_grid(input: &str) -> Grid<Cell> {
    let width = input
        .chars()
        .position(|c| c == '\n')
        .expect("input should contain more than 1 line") as u32;
    let height = input.lines().count() as u32;

    Grid::new_iterator(
        Size::new(width, height),
        input.chars().filter(|c| *c != '\n').map(Cell::from),
    )
}

fn find_start_position(input: &str) -> Option<Coord> {
    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '^' {
                return Some(Coord::new(
                    x.try_into().expect("should convert"),
                    y.try_into().expect("should convert"),
                ));
            }
        }
    }

    None
}

fn rotate_90(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::West => Direction::North,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
    }
}

fn next_position(
    grid: &Grid<Cell>,
    position: Coord,
    direction: Direction,
) -> Option<(Coord, Direction)> {
    let next = match direction {
        Direction::North => position + Coord::new(0, -1),
        Direction::West => position + Coord::new(-1, 0),
        Direction::East => position + Coord::new(1, 0),
        Direction::South => position + Coord::new(0, 1),
    };

    match grid.get(next) {
        Some(Cell::Empty) => Some((next, direction)),
        Some(Cell::Blocked) => next_position(grid, position, rotate_90(direction)),
        None => None,
    }
}

fn find_path(grid: &Grid<Cell>, mut position: Coord, mut direction: Direction) -> HashSet<Coord> {
    let mut seen_positions = HashSet::<Coord>::new();
    seen_positions.insert(position);

    while let Some((pos, dir)) = next_position(grid, position, direction) {
        position = pos;
        direction = dir;
        seen_positions.insert(position);
    }

    seen_positions
}

fn contains_loop(grid: &Grid<Cell>, mut position: Coord, mut direction: Direction) -> bool {
    let mut seen_pos_dir = HashSet::<(Coord, Direction)>::new();
    seen_pos_dir.insert((position, direction));

    while let Some((pos, dir)) = next_position(grid, position, direction) {
        position = pos;
        direction = dir;
        if !seen_pos_dir.insert((position, direction)) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let position = find_start_position(input).expect("must have start position");
    let direction = Direction::North;

    Some(find_path(&grid, position, direction).len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let position = find_start_position(input).expect("must have start position");
    let direction = Direction::North;

    let mut original_path = find_path(&grid, position, direction);
    original_path.remove(&position); // Can't modify start position
                                     //
    Some(
        original_path
            .par_iter()
            .filter(|pos| {
                let mut grid = grid.clone();
                *grid.get_mut(**pos).expect("should exist") = Cell::Blocked;

                contains_loop(&grid, position, direction)
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

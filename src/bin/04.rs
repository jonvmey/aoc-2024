use grid_2d::{Coord, Grid, Size};

advent_of_code::solution!(4);

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

fn check_xmas(grid: &Grid<char>, pos: Coord) -> u32 {
    let offsets = [
        Coord::new(-1, -1),
        Coord::new(0, -1),
        Coord::new(1, -1),
        Coord::new(-1, 0),
        Coord::new(1, 0),
        Coord::new(-1, 1),
        Coord::new(0, 1),
        Coord::new(1, 1),
    ];

    let mut valid_count = 0;

    if let Some('X') = grid.get(pos) {
        for offset in offsets {
            let pos = pos + offset;

            if let Some('M') = grid.get(pos) {
                let pos = pos + offset;

                if let Some('A') = grid.get(pos) {
                    let pos = pos + offset;

                    if let Some('S') = grid.get(pos) {
                        valid_count += 1;
                    }
                }
            }
        }
    }

    valid_count
}

fn check_x_mas(grid: &Grid<char>, pos: Coord) -> bool {
    if let Some('A') = grid.get(pos) {
        let top_left = grid.get(pos + Coord::new(-1, -1));
        let bottom_right = grid.get(pos + Coord::new(1, 1));

        let top_right = grid.get(pos + Coord::new(1, -1));
        let bottom_left = grid.get(pos + Coord::new(-1, 1));

        match (top_left, bottom_right) {
            (Some('M'), Some('S')) | (Some('S'), Some('M')) => matches!(
                (top_right, bottom_left),
                (Some('M'), Some('S')) | (Some('S'), Some('M'))
            ),
            _ => false,
        }
    } else {
        false
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);

    Some(
        grid.coord_iter()
            .map(|coord| check_xmas(&grid, coord))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);

    Some(
        grid.coord_iter()
            .filter(|coord| check_x_mas(&grid, *coord))
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}

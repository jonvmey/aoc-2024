use std::collections::{HashSet, VecDeque};

use grid_2d::{Coord, Grid, Size};

advent_of_code::solution!(12);

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    West,
    East,
    South,
}

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

fn adjacent_plots(plot: Coord) -> [Coord; 4] {
    [
        plot + Coord::new(1, 0),
        plot - Coord::new(1, 0),
        plot + Coord::new(0, 1),
        plot - Coord::new(0, 1),
    ]
}

fn is_edge_perimeter(plot: Coord, dir: Direction, farm: &Grid<char>) -> bool {
    let adj_plot = plot
        + match dir {
            Direction::North => Coord::new(0, -1),
            Direction::South => Coord::new(0, 1),
            Direction::West => Coord::new(-1, 0),
            Direction::East => Coord::new(1, 0),
        };

    if let Some(adj_type) = farm.get(adj_plot) {
        adj_type != farm.get(plot).unwrap()
    } else {
        true
    }
}

fn find_perimeter_edges(plot: Coord, farm: &Grid<char>) -> Vec<Direction> {
    let directions = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    directions
        .iter()
        .filter(|dir| is_edge_perimeter(plot, **dir, farm))
        .copied()
        .collect()
}

fn scan_region(plot: Coord, scanned: &mut Grid<bool>, farm: &Grid<char>) -> HashSet<Coord> {
    let mut to_scan = VecDeque::new();
    to_scan.push_back(plot);

    let mut region = HashSet::new();

    while let Some(plot) = to_scan.pop_front() {
        let plot_scanned = scanned.get_mut(plot).unwrap();

        if *plot_scanned {
            continue;
        } else {
            *plot_scanned = true;
        }

        region.insert(plot);

        let plot_type = farm.get(plot).unwrap();

        for adj_plot in adjacent_plots(plot) {
            if let Some(adj_type) = farm.get(adj_plot) {
                if adj_type == plot_type && !scanned.get(adj_plot).unwrap() {
                    to_scan.push_back(adj_plot);
                    region.insert(adj_plot);
                }
            }
        }
    }

    region
}

fn find_interior_corners(plot: Coord, farm: &Grid<char>) -> u32 {
    let edges = find_perimeter_edges(plot, farm);

    match edges.len() {
        0 | 1 => 0,
        2 => match (edges[0], edges[1]) {
            (Direction::North, Direction::South)
            | (Direction::South, Direction::North)
            | (Direction::West, Direction::East)
            | (Direction::East, Direction::West) => 0,
            _ => 1,
        },
        3 => 2,
        4 => 4,
        _ => panic!(),
    }
}

fn find_exterior_corners(plot: Coord, farm: &Grid<char>) -> u32 {
    let n = farm.get(plot + Coord::new(0, -1));
    let s = farm.get(plot + Coord::new(0, 1));
    let w = farm.get(plot + Coord::new(-1, 0));
    let e = farm.get(plot + Coord::new(1, 0));

    let nw = farm.get(plot + Coord::new(-1, -1));
    let ne = farm.get(plot + Coord::new(1, -1));
    let sw = farm.get(plot + Coord::new(-1, 1));
    let se = farm.get(plot + Coord::new(1, 1));

    let mut corners = 0;
    let plot_type = farm.get(plot).unwrap();

    if let Some(n) = n {
        if let Some(w) = w {
            if n == plot_type && w == plot_type && nw.unwrap() != plot_type {
                corners += 1;
            }
        }
        if let Some(e) = e {
            if n == plot_type && e == plot_type && ne.unwrap() != plot_type {
                corners += 1;
            }
        }
    }

    if let Some(s) = s {
        if let Some(w) = w {
            if s == plot_type && w == plot_type && sw.unwrap() != plot_type {
                corners += 1;
            }
        }
        if let Some(e) = e {
            if s == plot_type && e == plot_type && se.unwrap() != plot_type {
                corners += 1;
            }
        }
    }

    corners
}

fn calculate_perimeter(region: &HashSet<Coord>, farm: &Grid<char>) -> u32 {
    let mut perimeter = 0;

    for plot in region {
        let plot_type = farm.get(*plot).unwrap();

        for adj_plot in adjacent_plots(*plot) {
            if let Some(adj_type) = farm.get(adj_plot) {
                if adj_type != plot_type {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn calculate_sides(region: &HashSet<Coord>, farm: &Grid<char>) -> u32 {
    region
        .iter()
        .map(|plot| find_interior_corners(*plot, farm) + find_exterior_corners(*plot, farm))
        .sum()
}

fn calculate_region_price1(region: &HashSet<Coord>, farm: &Grid<char>) -> u32 {
    let area = region.len() as u32;
    let perimeter = calculate_perimeter(region, farm);

    area * perimeter
}

fn calculate_region_price2(region: &HashSet<Coord>, farm: &Grid<char>) -> u32 {
    let area = region.len() as u32;
    let sides = calculate_sides(region, farm);

    area * sides
}

fn calculate_fencing<F>(farm: &Grid<char>, calculate_region_price: F) -> u32
where
    F: Fn(&HashSet<Coord>, &Grid<char>) -> u32,
{
    let mut scanned = Grid::new_copy(farm.size(), false);
    let mut cost = 0;

    for plot in farm.coord_iter() {
        if !*scanned.get(plot).unwrap() {
            let region = scan_region(plot, &mut scanned, farm);
            cost += calculate_region_price(&region, farm);
        }
    }

    cost
}

pub fn part_one(input: &str) -> Option<u32> {
    let farm = parse_grid(input);

    Some(calculate_fencing(&farm, calculate_region_price1))
}

pub fn part_two(input: &str) -> Option<u32> {
    let farm = parse_grid(input);

    Some(calculate_fencing(&farm, calculate_region_price2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}

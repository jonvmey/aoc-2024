use std::collections::HashSet;

use grid_2d::{Coord, Grid, Size};

advent_of_code::solution!(10);

type Map = Grid<u8>;

fn parse_grid(input: &str) -> Map {
    let width = input
        .chars()
        .position(|c| c == '\n')
        .expect("input should contain more than 1 line") as u32;
    let height = input.lines().count() as u32;

    Grid::new_iterator(
        Size::new(width, height),
        input
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| c.to_digit(10).expect("shouldn't fail") as u8),
    )
}

fn adjacent_cells(position: Coord) -> [Coord; 4] {
    [
        position + Coord::new(-1, 0),
        position + Coord::new(1, 0),
        position + Coord::new(0, -1),
        position + Coord::new(0, 1),
    ]
}

fn get_trailheads(map: &Map) -> Vec<Coord> {
    map.coord_iter()
        .filter(|p| *map.get(*p).expect("shouldn't fail") == 0)
        .collect()
}

fn find_unique_trails(p: Coord, map: &Map, mut path: Vec<Coord>) -> HashSet<Vec<Coord>> {
    let mut paths = HashSet::new();
    let elevation = map.get(p).expect("must exist");
    path.push(p);

    if *elevation == 9 {
        paths.insert(path);
    } else {
        for adjacent in adjacent_cells(p) {
            if let Some(adj_elevation) = map.get(adjacent) {
                if *adj_elevation == elevation + 1 {
                    paths.extend(find_unique_trails(adjacent, map, path.clone()));
                }
            }
        }
    }

    paths
}

fn calc_trail_score(trailhead: Coord, map: &Map) -> u32 {
    let trail_ends: HashSet<Coord> = find_unique_trails(trailhead, map, Vec::new())
        .into_iter()
        .map(|path| *path.last().unwrap())
        .collect();

    trail_ends.len() as u32
}

fn calc_trail_rating(trailhead: Coord, map: &Map) -> u32 {
    find_unique_trails(trailhead, map, Vec::new()).len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_grid(input);

    Some(
        get_trailheads(&map)
            .into_iter()
            .map(|trailhead| calc_trail_score(trailhead, &map))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_grid(input);

    Some(
        get_trailheads(&map)
            .into_iter()
            .map(|trailhead| calc_trail_rating(trailhead, &map))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}

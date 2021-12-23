use std::collections::BTreeSet;

use itertools::Itertools;

type Coords = (i32, i32);
type Basin = BTreeSet<Coords>;

struct Grid {
    values: Vec<u8>,
    width: i32,
    height: i32,
}

impl Grid {
    fn get(&self, (x, y): Coords) -> Option<u8> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(self.values[(x + y * self.width) as usize])
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Grid {
    let width = input.lines().next().unwrap().chars().count() as i32;
    let height = input.lines().count() as i32;
    Grid {
        values: input.lines().flat_map(|line| line.bytes()).collect(),
        width,
        height,
    }
}

fn is_minima(grid: &Grid, coords: Coords) -> bool {
    match grid.get(coords) {
        Some(height) => neighbours(coords)
            .iter()
            .flat_map(|&d| grid.get(d))
            .all(|val| val > height),
        _ => false,
    }
}

fn matching_coords<T>(rows: &Grid, predicate: T) -> Vec<Coords>
where
    T: Fn(&Grid, Coords) -> bool,
{
    (0..rows.width)
        .flat_map(|i| (0..rows.height).map(move |j| (i, j)))
        .filter(|&coords| predicate(rows, coords))
        .collect()
}

fn minima_coords(rows: &Grid) -> Vec<Coords> {
    matching_coords(rows, is_minima)
}

#[aoc(day9, part1)]
pub fn part_1(input: &str) -> u32 {
    let rows = &parse(input);
    minima_coords(rows)
        .iter()
        .flat_map(|(i, j)| rows.get((*i, *j)))
        .map(|height| (height - b'0' + 1) as u32)
        .sum()
}

fn neighbours((i, j): Coords) -> [Coords; 4] {
    [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]
}

fn foreach_sub_9_neighbour<T>(rows: &Grid, coords: Coords, foreach: T)
where
    T: FnMut(&Coords),
{
    neighbours(coords)
        .iter()
        .filter(|&&n| matches!(rows.get(n), Some(val) if val < b'9'))
        .for_each(foreach)
}

fn flood_fill(basin: &mut Basin, rows: &Grid, coords: Coords) {
    if basin.insert(coords) {
        foreach_sub_9_neighbour(rows, coords, |n| flood_fill(basin, rows, *n))
    }
}

fn basin_size(rows: &Grid, coords: Coords) -> usize {
    let mut basin = Basin::new();
    flood_fill(&mut basin, rows, coords);
    basin.len()
}

#[aoc(day9, part2)]
pub fn part_2(input: &str) -> usize {
    let rows = &parse(input);
    minima_coords(rows)
        .iter()
        .map(|coords| basin_size(rows, *coords))
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(3)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

        assert_eq!(15, part_1(input));
        assert_eq!(1134, part_2(input));
    }
}

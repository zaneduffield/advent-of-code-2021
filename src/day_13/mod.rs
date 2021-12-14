use rustc_hash::FxHashSet;
use std::cmp::max;

use itertools::Itertools;

type Coords = (usize, usize);

#[derive(Clone, Copy)]
struct Fold {
    is_1st_dim: bool,
    val: usize,
}

#[derive(Clone)]
struct Grid {
    values: FxHashSet<Coords>,
    folds: Vec<Fold>,
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Grid {
    let mut folds = vec![];
    let mut values = FxHashSet::default();
    for line in input.lines().filter(|s| !s.is_empty()) {
        match line.strip_prefix("fold along ") {
            Some(fold) => {
                let (axis, val) = fold.split_once("=").unwrap();
                folds.push(Fold {
                    is_1st_dim: axis == "y",
                    val: val.parse().unwrap(),
                })
            }
            None => {
                let (x, y) = line.split_once(",").unwrap();
                values.insert((y.parse().unwrap(), x.parse().unwrap()));
            }
        }
    }

    folds.reverse();
    Grid { values, folds }
}

fn folded_dest(fold: Fold, p: Coords) -> Option<Coords> {
    let mut coord = if fold.is_1st_dim { p.0 } else { p.1 };
    if coord <= fold.val {
        Some(p)
    } else if 2 * fold.val >= coord {
        coord = 2 * fold.val - coord;
        if fold.is_1st_dim {
            Some((coord, p.1))
        } else {
            Some((p.0, coord))
        }
    } else {
        None
    }
}

fn fold_grid(grid: &mut Grid) {
    let fold = match grid.folds.pop() {
        None => return,
        Some(v) => v,
    };

    grid.values = grid
        .values
        .iter()
        .flat_map(|&p| folded_dest(fold, p))
        .collect();
}

fn render(grid: &Grid) -> String {
    let max_1 = grid.values.iter().fold(0, |a, p| max(a, p.1));

    let mut out = "\n".to_string();
    let mut cur_row = 0;
    for (row, g) in &grid.values.iter().sorted().group_by(|p| p.0) {
        while row > cur_row {
            out += "\n";
            cur_row += 1;
        }

        let mut s = ".".repeat(max_1 + 1);
        g.for_each(|p| s.replace_range(p.1..p.1 + 1, "#"));
        out += &s;
    }

    out
}

#[aoc(day13, part1)]
fn part_1(grid: &Grid) -> usize {
    let mut grid = grid.to_owned();
    fold_grid(&mut grid);
    grid.values.len()
}

#[aoc(day13, part2)]
fn part_2(grid: &Grid) -> String {
    let mut grid = grid.to_owned();
    while !grid.folds.is_empty() {
        fold_grid(&mut grid);
    }
    render(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

        assert_eq!(17, part_1(&parse(input)));
        assert_eq!(
            "
#####
#...#
#...#
#...#
#####",
            part_2(&parse(input))
        );
    }
}

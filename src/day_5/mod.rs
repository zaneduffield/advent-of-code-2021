use std::cmp::{max, min};

type Grid = Vec<Vec<u64>>;
type Pair = (usize, usize);

#[derive(Copy, Clone)]
struct Line {
    start: Pair,
    end: Pair,
}

fn parse_coords(coords: &str) -> Pair {
    let (start, end) = coords.split_once(",").unwrap();
    (
        start.parse::<usize>().unwrap(),
        end.parse::<usize>().unwrap(),
    )
}

fn parse(contents: &str) -> Vec<Line> {
    contents
        .lines()
        .map(|line| {
            line.split_once(" -> ")
                .map(|pairs| Line {
                    start: parse_coords(pairs.0),
                    end: parse_coords(pairs.1),
                })
                .unwrap()
        })
        .collect()
}

fn add_dim_0_line(grid: &mut Grid, line: Line) {
    for row in &mut grid[min(line.start.0, line.end.0)..=max(line.start.0, line.end.0)] {
        row[line.start.1] += 1;
    }
}

fn add_dim_1_line(grid: &mut Grid, line: Line) {
    let row = &mut grid[line.start.0];
    for col in &mut row[min(line.start.1, line.end.1)..=max(line.start.1, line.end.1)] {
        *col += 1;
    }
}

fn add_diag_line(grid: &mut Grid, line: Line) {
    let (mut x, mut y) = (line.start.0, line.start.1);

    grid[x][y] += 1;
    while (x, y) != line.end {
        if line.end.0 > line.start.0 {
            x += 1
        } else {
            x -= 1
        }
        if line.end.1 > line.start.1 {
            y += 1
        } else {
            y -= 1
        }
        grid[x][y] += 1;
    }
}

fn make_grid(lines: &[Line]) -> Grid {
    let (max_0, max_1) = lines
        .iter()
        .fold((0, 0), |(max_0, max_1), Line { start, end }| {
            (
                max(max(max_0, start.0), end.0),
                max(max(max_1, start.1), end.1),
            )
        });
    (0..=max_0).map(|_| vec![0; max_1 + 1]).collect()
}

fn grid_count(grid: Grid, min_val: u64) -> usize {
    grid.iter()
        .map(|row| row.iter().filter(|n| **n >= min_val).count())
        .sum()
}

fn solve(lines: &[Line], handle_diag_line: impl Fn(&mut Grid, Line)) -> usize {
    let mut grid = make_grid(lines);

    lines.iter().for_each(|line| {
        if line.start.0 == line.end.0 {
            add_dim_1_line(&mut grid, *line);
        } else if line.start.1 == line.end.1 {
            add_dim_0_line(&mut grid, *line);
        } else {
            handle_diag_line(&mut grid, *line);
        }
    });

    grid_count(grid, 2)
}

#[aoc(day5, part1)]
pub fn part_1(input: &str) -> usize {
    let lines = parse(input);
    solve(&lines, |_, _| {})
}

#[aoc(day5, part2)]
pub fn part_2(input: &str) -> usize {
    let lines = parse(input);
    solve(&lines, add_diag_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";
        assert_eq!(5, part_1(input));
        assert_eq!(12, part_2(input));

        let input = "1,1 -> 3,3\n3,3 -> 1,1";
        assert_eq!(part_2(input), 3);
    }
}

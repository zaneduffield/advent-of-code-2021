use pathfinding::prelude::Matrix;

fn parse(input: &str) -> Matrix<u8> {
    input.lines().map(|line| line.bytes()).collect()
}

fn step(grid: &mut Matrix<u8>) -> bool {
    let mut changed = false;
    for is_hor in [true, false] {
        let mut new_grid = grid.clone();
        let (s, rotations) = if is_hor { (b'>', 3) } else { (b'v', 1) };
        for i @ (r, c) in grid.indices() {
            if grid[i] == s && grid[(r, (c + 1) % grid.columns)] == b'.' {
                new_grid.swap(grid.idx(i), grid.idx((r, (c + 1) % grid.columns)));
                changed = true;
            }
        }
        *grid = new_grid.rotated_cw(rotations);
    }
    changed
}

fn solve(grid: &mut Matrix<u8>) -> usize {
    (0..).position(|_| !step(grid)).unwrap() + 1
}

#[aoc(day25, part1)]
pub fn part_1(input: &str) -> usize {
    solve(&mut parse(input))
}

#[aoc(day25, part2)]
pub fn part_2(_: &str) -> String {
    String::new()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let input = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
        assert_eq!(58, part_1(input));
    }
}

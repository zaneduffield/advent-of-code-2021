const WIDTH: usize = 10;
const HEIGHT: usize = 10;
type Grid = [[u8; HEIGHT]; WIDTH];

type Coords = (usize, usize);

#[aoc_generator(day11)]
fn parse(input: &str) -> Grid {
    let mut out = [[0; HEIGHT]; WIDTH];

    for (i, line) in input.lines().enumerate().take(WIDTH) {
        for (j, c) in line.bytes().enumerate().take(HEIGHT) {
            out[i][j] = c - b'0';
        }
    }
    out
}

fn neighbours((i, j): Coords) -> Vec<Coords> {
    let mut out = Vec::with_capacity(8);
    let (i_i32, j_i32) = (i as i32, j as i32);
    for x in i_i32 - 1..=i_i32 + 1 {
        for y in j_i32 - 1..=j_i32 + 1 {
            if (x, y) != (i_i32, j_i32)
                && (0..WIDTH as i32).contains(&x)
                && (0..HEIGHT as i32).contains(&y)
            {
                out.push((x as usize, y as usize))
            }
        }
    }
    out
}

fn inc_if_nonzero(x: &mut u8) {
    if *x != 0 {
        *x += 1;
    }
}

fn flash(grid: &mut Grid, (i, j): Coords) -> u64 {
    grid[i][j] = 0;
    let nbours = neighbours((i, j));
    nbours
        .iter()
        .for_each(|&(x, y)| inc_if_nonzero(&mut grid[x][y]));

    1 + nbours
        .iter()
        .map(|&(x, y)| flash_if_full(grid, (x, y)))
        .sum::<u64>()
}

fn flash_if_full(grid: &mut Grid, (i, j): Coords) -> u64 {
    match grid[i][j] {
        x if x <= 9 || x == 0 => 0,
        _ => flash(grid, (i, j)),
    }
}

fn generation(grid: &mut Grid) -> u64 {
    grid.iter_mut()
        .for_each(|row| row.iter_mut().for_each(|c| *c += 1));

    (0..WIDTH).fold(0, |sum, row| {
        sum + (0..HEIGHT).fold(0, |sum, col| sum + flash_if_full(grid, (row, col)))
    })
}

#[aoc(day11, part1)]
fn part_1(grid: &Grid) -> u64 {
    let mut grid = *grid;
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += generation(&mut grid)
    }

    flashes
}

#[aoc(day11, part2)]
fn part_2(grid: &Grid) -> usize {
    let mut grid = *grid;
    1 + (0..)
        .position(|_| generation(&mut grid) == (WIDTH * HEIGHT) as u64)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        assert_eq!(1656, part_1(&parse(input)));
        assert_eq!(195, part_2(&parse(input)));
    }
}

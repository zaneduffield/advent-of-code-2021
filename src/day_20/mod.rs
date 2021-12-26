use std::{fmt::Display, mem};

struct Image {
    lookup: Vec<bool>,
    data: Vec<bool>,
    max_size: (usize, usize),
    on_at_inf: bool,
}

fn bits_to_u16<I: Iterator<Item = usize>>(iter: I) -> usize {
    iter.fold(0, |result, bit| (result << 1) ^ bit)
}

impl Image {
    fn new(input: &str, num_iter: u8) -> Image {
        let lookup = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| c == '#')
            .collect();

        let buff = (num_iter + 1) as usize;
        let width = input.lines().nth(2).unwrap().len();
        let max_width = width + 2 * buff;

        let mut data = Vec::with_capacity((input.len() + 2 * buff) * max_width);
        data.extend((0..buff * max_width).map(|_| false));
        for line in input.lines().skip(2).filter(|line| !line.is_empty()) {
            data.extend((0..buff).map(|_| false));
            data.extend(line.chars().map(|c| c == '#'));
            data.extend((0..buff).map(|_| false));
        }
        data.extend((0..buff * max_width).map(|_| false));

        let max_height = data.len() / max_width;

        Image {
            lookup,
            data,
            max_size: (max_width, max_height),
            on_at_inf: false,
        }
    }

    #[inline]
    fn idx(&self, (x, y): (usize, usize)) -> usize {
        y * self.max_size.1 + x
    }

    #[inline]
    fn get(&self, (x, y): (i32, i32)) -> bool {
        if x >= 0 && y >= 0 {
            let idx = self.idx((x as usize, y as usize));
            if idx < self.data.len() {
                return self.data[idx as usize];
            }
        }
        self.on_at_inf && self.lookup[0]
    }

    fn enhance_grid_at(&self, (x, y): (usize, usize)) -> bool {
        let (x, y) = (x as i32, y as i32);
        let index: usize = bits_to_u16(
            [
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y - 1),
                (x - 1, y),
                (x, y),
                (x + 1, y),
                (x - 1, y + 1),
                (x, y + 1),
                (x + 1, y + 1),
            ]
            .iter()
            .map(|c| self.get(*c) as usize),
        );

        self.lookup[index]
    }

    fn enhance(&mut self, num_iter: u8) {
        let mut other = vec![false; self.data.len()];
        for _ in 0..num_iter {
            for y in 0..self.max_size.1 {
                for x in 0..self.max_size.0 {
                    other[self.idx((x, y))] = self.enhance_grid_at((x, y));
                }
            }
            mem::swap(&mut self.data, &mut other);
            self.on_at_inf = !self.on_at_inf;
        }
    }

    fn count_lit(&self) -> usize {
        self.data.iter().filter(|b| **b).count()
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        let width = self.max_size.0;
        for y in 0..self.max_size.1 {
            out.extend(self.data.iter().skip(y * width).take(width).map(|b| {
                if *b {
                    '#'
                } else {
                    '.'
                }
            }));
            out.push('\n');
        }
        write!(f, "{}", out)
    }
}

fn solve(input: &str, num_iter: u8) -> usize {
    let mut image = Image::new(input, num_iter);
    image.enhance(num_iter);
    image.count_lit()
}

#[aoc(day20, part1)]
pub fn part_1(input: &str) -> usize {
    solve(input, 2)
}

#[aoc(day20, part2)]
pub fn part_2(input: &str) -> usize {
    solve(input, 50)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        assert_eq!(35, part_1(input));
        assert_eq!(3351, part_2(input));
    }
}

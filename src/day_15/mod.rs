use std::{cmp::Reverse, collections::BinaryHeap};

type Coords = (usize, usize);
struct Grid {
    width: usize,
    height: usize,
    risks: Vec<u8>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    potential_cost: usize,
    current_cost: usize,
    coords: Coords,
}

impl Grid {
    fn idx(&self, c: Coords) -> usize {
        c.0 * self.width + c.1
    }

    fn risk(&self, c: Coords) -> u8 {
        self.risks[self.idx(c)]
    }

    fn neighbours(&self, c: Coords) -> [Option<Coords>; 4] {
        let mut out = [None; 4];
        let mut count = 0..;
        let (i, j) = (c.0 as i32, c.1 as i32);
        for (x, y) in [(i, j + 1), (i, j - 1), (i + 1, j), (i - 1, j)] {
            if (0..self.width as i32).contains(&x) && (0..self.height as i32).contains(&y) {
                out[count.next().unwrap()] = Some((x as usize, y as usize));
            }
        }
        out
    }

    fn solve(&self) -> Option<usize> {
        let mut best_current_costs = vec![usize::MAX; self.risks.len()];
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(Node {
            potential_cost: self.width + self.height,
            current_cost: 0,
            coords: (0, 0),
        }));

        loop {
            let next = match heap.pop() {
                None => return None,
                Some(next) => next.0,
            };
            if next.coords == (self.height - 1, self.width - 1) {
                return Some(next.current_cost);
            }
            for &coords in self.neighbours(next.coords).iter().flatten() {
                let current_cost = next.current_cost + self.risk(coords) as usize;
                let idx = self.idx(coords);
                if current_cost < best_current_costs[idx] {
                    best_current_costs[idx] = current_cost;
                    let node = Node {
                        potential_cost: current_cost
                            + (self.height - coords.0)
                            + (self.width - coords.1),
                        current_cost,
                        coords,
                    };
                    heap.push(Reverse(node));
                }
            }
        }
    }
}

fn parse(input: &str) -> Grid {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().bytes().len();
    let risks = input
        .lines()
        .flat_map(|line| line.bytes().map(|b| b - b'0'))
        .collect();

    Grid {
        width,
        height,
        risks,
    }
}

fn expand(grid: Grid, factor: usize) -> Grid {
    let new_risks = vec![0; factor * factor * grid.risks.len()];
    let mut new_grid = Grid {
        width: factor * grid.width,
        height: factor * grid.height,
        risks: new_risks,
    };

    for i in 0..new_grid.height {
        for j in 0..new_grid.width {
            let base_risk = grid.risk((i % grid.height, j % grid.width));
            let new_risk =
                1 + (base_risk - 1 + (i / grid.height) as u8 + (j / grid.width) as u8) % 9;
            let idx = new_grid.idx((i, j));
            new_grid.risks[idx] = std::cmp::max(1, new_risk);
        }
    }

    new_grid
}

#[aoc(day15, part1)]
pub fn part_1(input: &str) -> usize {
    let grid = parse(input);
    grid.solve().expect("A path to the end should exist")
}

#[aoc(day15, part2)]
pub fn part_2(input: &str) -> usize {
    let grid = expand(parse(input), 5);
    grid.solve().expect("A path to the end should exist")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        assert_eq!(40, part_1(input));
        assert_eq!(315, part_2(input));
    }
}

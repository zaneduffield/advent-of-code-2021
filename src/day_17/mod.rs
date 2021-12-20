use std::ops::Range;

use itertools::Itertools;

const START_Y: i32 = 0;
const START_X: i32 = 0;

struct Target {
    x_range: Range<i32>,
    y_range: Range<i32>,
}

impl Target {
    fn in_range(&self, (x, y): (i32, i32)) -> bool {
        self.x_range.contains(&x) && self.y_range.contains(&y)
    }

    fn max_y_vel(&self) -> i32 {
        self.y_range.end.max(i32::abs(self.y_range.start) - 1)
    }

    fn lo_y_vel(&self) -> i32 {
        -self.max_y_vel() - 1
    }

    fn lo_x_vel(&self) -> i32 {
        0.min(self.x_range.start - START_X)
    }

    fn hi_x_vel(&self) -> i32 {
        0.max(self.x_range.end - START_X)
    }

    fn is_hit_from(&self, (mut x_v, mut y_v): (i32, i32)) -> bool {
        let (mut x, mut y) = (START_X, START_Y);
        loop {
            if self.in_range((x, y)) {
                return true;
            }
            x += x_v;
            y += y_v;
            y_v -= 1;
            x_v += -x_v.signum();

            if y < self.y_range.start && y_v <= 0 {
                return false;
            }
        }
    }
}

fn parse(mut input: &str) -> Target {
    input = input.strip_prefix("target area: ").unwrap();
    let (x_range, y_range) = input.split_once(", ").unwrap();

    let parse_pair = |input: &str, prefix: &str| {
        input
            .strip_prefix(prefix)
            .unwrap()
            .split("..")
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap()
    };

    let (x1, x2) = parse_pair(x_range, "x=");
    let (y1, y2) = parse_pair(y_range, "y=");

    Target {
        x_range: x1..(x2 + 1),
        y_range: y1..(y2 + 1),
    }
}

#[aoc(day17, part1)]
fn part_1(input: &str) -> i32 {
    let target = parse(input);
    let max_init_y_vel = target.max_y_vel();
    max_init_y_vel * (max_init_y_vel + 1) / 2
}

#[aoc(day17, part2)]
fn part_2(input: &str) -> u32 {
    let target = parse(input);
    let mut count = 0;
    for x_v in target.lo_x_vel()..=target.hi_x_vel() {
        for y_v in target.lo_y_vel()..=target.max_y_vel() {
            count += target.is_hit_from((x_v, y_v)) as u32;
        }
    }

    count
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let input = "target area: x=20..30, y=-10..-5";
        assert_eq!(45, part_1(input));
        assert_eq!(112, part_2(input));
    }
}

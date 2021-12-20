use std::{fmt, iter::Peekable};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum SnailfishNum {
    Literal(i32),
    Pair(Box<SnailfishNum>, Box<SnailfishNum>),
}

enum Side {
    Left,
    Right,
}

impl fmt::Display for SnailfishNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal(v) => write!(f, "{}", v),
            Pair(left, right) => write!(f, "[{},{}]", left, right),
        }
    }
}

use SnailfishNum::*;

type Residue = (i32, i32);

impl SnailfishNum {
    fn magnitude(&self) -> i32 {
        match self {
            Literal(v) => *v,
            Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn add(&self, other: &Self) -> Self {
        Self::Pair(Box::new(self.clone()), Box::new(other.clone()))
    }

    fn reduce(mut self) -> Self {
        loop {
            if self.explode(0).is_none() && !self.split() {
                break;
            }
        }
        self
    }

    fn split(&mut self) -> bool {
        match self {
            Literal(val) if *val >= 10 => {
                let (left, right) = (*val / 2, (*val + 1) / 2);
                *self = Pair(Box::new(Literal(left)), Box::new(Literal(right)));
                true
            }
            Pair(left, right) => left.split() || right.split(),
            _ => false,
        }
    }

    fn explode(&mut self, depth: u32) -> Option<Residue> {
        if depth >= 4 {
            let residue = self.residue();
            if residue.is_some() {
                *self = Literal(0);
                return residue;
            }
        }

        match self {
            Pair(left, right) => {
                if let Some(residue) = left.explode(depth + 1) {
                    right.apply_residue(residue.1, Side::Left);
                    Some((residue.0, 0))
                } else if let Some(residue) = right.explode(depth + 1) {
                    left.apply_residue(residue.0, Side::Right);
                    Some((0, residue.1))
                } else {
                    None
                }
            }
            Literal(_) => None,
        }
    }

    fn value(&self) -> Option<i32> {
        if let Literal(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    fn residue(&self) -> Option<Residue> {
        if let Pair(left, right) = self {
            if let (Some(val_left), Some(val_right)) = (left.value(), right.value()) {
                return Some((val_left, val_right));
            }
        }
        None
    }

    fn apply_residue(&mut self, val: i32, side: Side) {
        match self {
            Literal(old) => {
                *self = Literal(*old + val);
            }
            Pair(left, right) => match side {
                Side::Left => left.apply_residue(val, side),
                Side::Right => right.apply_residue(val, side),
            },
        }
    }
}

fn parse_literal<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> SnailfishNum {
    let num = iter
        .peeking_take_while(|c| c.is_digit(10) || c == &'-')
        .collect::<String>();
    SnailfishNum::Literal(num.parse().unwrap())
}

fn parse_pair<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> SnailfishNum {
    if Some(&'[') == iter.peek() {
        iter.next();
        let left = parse_pair(iter);
        iter.next();
        let right = parse_pair(iter);
        iter.next();
        SnailfishNum::Pair(Box::new(left), Box::new(right))
    } else {
        parse_literal(iter)
    }
}

fn parse(input: &str) -> Vec<SnailfishNum> {
    input
        .lines()
        .map(|line| parse_pair(&mut line.chars().peekable()))
        .collect()
}

#[aoc(day18, part1)]
fn part_1(input: &str) -> i32 {
    let nums = parse(input);
    let sum = nums.into_iter().reduce(|a, b| a.add(&b).reduce()).unwrap();
    sum.magnitude()
}

#[aoc(day18, part2)]
fn part_2(input: &str) -> i32 {
    let nums = parse(input);
    (0..nums.len())
        .flat_map(|i| (0..nums.len()).map(move |j| (i, j)))
        .map(|(i, j)| nums[i].add(&nums[j]).reduce().magnitude())
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let input = "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";
        assert_eq!(4140, part_1(input));
        assert_eq!(3993, part_2(input));
    }
}

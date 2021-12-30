use itertools::Itertools;

// as explained here https://www.reddit.com/r/adventofcode/comments/rnejv5/comment/hps5hgw/?utm_source=share&utm_medium=web2x&context=3

/// The entire input is in this form repeated 14 times:

/// inp w
/// mul x 0
/// add x z
/// mod x 26
/// div z {a}
/// add x {b}
/// eql x w
/// eql x 0
/// mul y 0
/// add y 25
/// mul y x
/// add y 1
/// mul z y
/// mul y 0
/// add y w
/// add y {c}
/// mul y x
/// add z y
/// This in decompiled Python is

/// w = int(input())
/// x = int((z % 26) + b != w)
/// z //= a
/// z *= 25*x+1
/// z += (w+c)*x
/// Another thing to note is that the a is 1 seven times and 26 the other seven times. In the block where a is 1, b is always between 10 and 16. It follows that z //= {a} line is no-op and (z % 26) + b != w is always true. So the decompiled code becomes:

/// w = int(input())
/// z *= 26
/// z += w+c
/// So this block of code is "pushing" a digit of w+c in base 26. So to get 0 at the end, we have to "pop" these digits back out using z //= 26 and don't add any more back. Thus, in the lines with a=26, x = int((z % 26) + b != w) must be 0, which means the last pushed digit w_old+c must be equal to w_now-b.

/// For my particular input, it meant that

/// I[2]+ 6-14 == I[3]
/// I[4]+ 9- 7 == I[5]
/// I[8]+ 1- 7 == I[9]
/// I[7]+ 3- 8 == I[10]
/// I[6]+14- 7 == I[11]
/// I[1]+ 5- 5 == I[12]
/// I[0]+15-10 == I[13]
/// where I is the array of input.

fn parse_last_num(line: &str) -> i64 {
    line.split_whitespace().last().unwrap().parse().unwrap()
}

fn solve(input: &str) -> (i64, i64) {
    let lines = input.lines().collect_vec();
    let mut stack = vec![];
    let (mut max, mut min) = (99999999999999, 11111111111111);

    for i in 0..14 {
        let a = parse_last_num(lines[18 * i + 5]);
        let b = parse_last_num(lines[18 * i + 15]);

        if a > 0 {
            stack.push((i, b));
            continue;
        }
        let (j, b) = stack.pop().unwrap();

        max -= ((a + b) * 10_i64.pow((13 - if a > -b { j } else { i }) as u32)).abs();
        min += ((a + b) * 10_i64.pow((13 - if a < -b { j } else { i }) as u32)).abs();
    }

    (max, min)
}

#[aoc(day24, part1)]
pub fn part_1(input: &str) -> i64 {
    solve(input).0
}

#[aoc(day24, part2)]
pub fn part_2(input: &str) -> i64 {
    solve(input).1
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let input = include_str!("../../input/2021/day24.txt");
        assert_eq!(91297395919993, part_1(input));
        assert_eq!(71131151917891, part_2(input));
    }
}

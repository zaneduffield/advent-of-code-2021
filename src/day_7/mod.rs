type Position = i64;

fn parse(input: &str) -> Vec<Position> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse::<Position>().unwrap())
        .collect()
}

fn abs_diff<T: std::ops::Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}

fn triangle_diff(x: Position, y: Position) -> Position {
    let diff = abs_diff(x, y);
    diff * (diff + 1) / 2
}

fn take_diff_sum<F>(vals: &[Position], take: usize, target: Position, map: F) -> Position
where
    F: Fn(Position, Position) -> Position,
{
    vals.iter().take(take).map(|n| map(*n, target)).sum()
}

fn solve<F>(crabs: &[Position], diff_cost: F) -> i64
where
    F: Fn(Position, Position) -> Position + Copy,
{
    let mut best_pos = crabs[0];
    let mut best_sum = 0;
    for i in 0..crabs.len() {
        best_sum += diff_cost(crabs[i], best_pos);
        let diff = (crabs[i] - best_pos).signum();
        let mut pos = best_pos;
        loop {
            pos += diff;
            let sum = take_diff_sum(crabs, i + 1, pos, diff_cost);
            if sum < best_sum {
                best_pos = pos;
                best_sum = sum;
            } else {
                break;
            }
        }
    }

    best_sum
}

pub fn part_1(input: &str) -> i64 {
    let crabs = parse(input);
    solve(&crabs, abs_diff)
}

pub fn part_2(input: &str) -> i64 {
    let crabs = parse(input);
    solve(&crabs, triangle_diff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(37, part_1(input));
        assert_eq!(168, part_2(input));
    }
}

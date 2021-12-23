const MAX_AGE: usize = 8;
const REBIRTH_AGE: usize = 6;
type Fishies = [u64; MAX_AGE + 1];

fn population_after_generations(fishies: &Fishies, generations: i32) -> u64 {
    let mut fishies = *fishies;
    for _ in 0..generations {
        fishies.rotate_left(1);
        fishies[REBIRTH_AGE] += fishies[MAX_AGE];
    }

    fishies.iter().sum()
}

fn parse(input: &str) -> Fishies {
    let mut counts = [0; MAX_AGE + 1];
    input
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().expect(&format!("invalid input: {}", n)))
        .for_each(|n| counts[n] += 1);
    counts
}

#[aoc(day6, part1)]
pub fn part_1(input: &str) -> u64 {
    let fishies = parse(input);
    population_after_generations(&fishies, 80)
}

#[aoc(day6, part2)]
pub fn part_2(input: &str) -> u64 {
    let fishies = parse(input);
    population_after_generations(&fishies, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "3,4,3,1,2";
        assert_eq!(5934, part_1(input));
        assert_eq!(26984457539, part_2(input));
    }
}

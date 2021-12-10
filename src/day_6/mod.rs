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

#[aoc_generator(day6)]
fn parse(input: &str) -> Fishies {
    let mut counts = [0; MAX_AGE + 1];
    input
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .for_each(|n| counts[n] += 1);
    counts
}

#[aoc(day6, part1)]
fn part_1(fishies: &Fishies) -> u64 {
    population_after_generations(fishies, 80)
}

#[aoc(day6, part2)]
fn part_2(fishies: &Fishies) -> u64 {
    population_after_generations(fishies, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(5934, part_1(&parse("3,4,3,1,2")));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(26984457539, part_2(&parse("3,4,3,1,2")));
    }
}

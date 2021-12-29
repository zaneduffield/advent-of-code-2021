use std::mem;

use itertools::Itertools;

const BOARD_LEN: usize = 10;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
struct Player {
    pos: usize,
    score: usize,
}

impl Player {
    fn new(pos: usize) -> Player {
        Player { pos, score: 0 }
    }
    fn advance(&mut self, n: usize) {
        // -1 then +1 handles 1-based board indexing
        self.pos = (self.pos + n - 1) % BOARD_LEN + 1;
        self.score += self.pos;
    }
}

struct DeterministicDie {
    n: usize,
    max: usize,
}

impl Iterator for DeterministicDie {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.n + 1;
        self.n = next % self.max;
        Some(next)
    }
}

fn parse(input: &str) -> (Player, Player) {
    let (p1, p2) = input
        .lines()
        .map(|line| line[line.len() - 2..].trim().parse().unwrap())
        .collect_tuple()
        .unwrap();
    (Player::new(p1), Player::new(p2))
}

fn score_part_1((mut p1, mut p2): (Player, Player)) -> usize {
    const ROLLS_PER_TURN: usize = 3;
    const WINNING_SCORE: usize = 1000;

    let die = &mut DeterministicDie { n: 0, max: 100 };
    let mut roll_num = 0;
    loop {
        let roll_total: usize = die.take(ROLLS_PER_TURN).sum();
        roll_num += ROLLS_PER_TURN;
        p1.advance(roll_total);
        if p1.score >= WINNING_SCORE {
            return p2.score * roll_num;
        }
        mem::swap(&mut p1, &mut p2);
    }
}

const MAX_GAME_LEN: usize = 11;

struct UniverseCounts {
    num_won_by_turn: [usize; MAX_GAME_LEN],
    num_not_won_by_turn: [usize; MAX_GAME_LEN],
}

fn universe_counts(player: Player) -> UniverseCounts {
    // this holds frequencies indexed by roll totals (less the offset)
    const ROLL_COUNTS: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];
    const ROLL_COUNT_OFFSET: usize = 2;
    const MAX_P2_SCORE: usize = 21;

    // the idea here is to maintain a grid of scores tallies for each position at each turn,
    // and build it from start to finish in a dynamic programming style.
    // The key insight is that for each position and score on a turn, you can iterate over the potential rolls:
    // next_turn[next_pos][next_score] += frequency(roll) * old_turn[old_pos][old_score]
    let mut scores = [[[0; MAX_P2_SCORE + 1]; BOARD_LEN + 1]; MAX_GAME_LEN];
    scores[0][player.pos][0] = 1;
    (1..MAX_GAME_LEN).for_each(|turn| {
        (1..BOARD_LEN + 1).for_each(|pos| {
            (0..MAX_P2_SCORE).for_each(|score| {
                ROLL_COUNTS.iter().enumerate().for_each(|(roll, freq)| {
                    let new_pos = (pos + roll + ROLL_COUNT_OFFSET) % BOARD_LEN + 1;
                    let new_score = (score + new_pos).min(MAX_P2_SCORE);
                    scores[turn][new_pos][new_score] += freq * scores[turn - 1][pos][score];
                })
            })
        })
    });

    let mut result = UniverseCounts {
        num_not_won_by_turn: [0; MAX_GAME_LEN],
        num_won_by_turn: [0; MAX_GAME_LEN],
    };
    scores.iter().enumerate().for_each(|(turn, turn_data)| {
        turn_data[1..].iter().for_each(|pos_scores| {
            result.num_not_won_by_turn[turn] += pos_scores.iter().take(MAX_P2_SCORE).sum::<usize>();
            result.num_won_by_turn[turn] += pos_scores[MAX_P2_SCORE];
        })
    });
    result
}

fn sum_product<I, T>(a: I, b: T) -> usize
where
    T: Iterator<Item = usize>,
    I: Iterator<Item = usize>,
{
    a.zip(b).fold(0, |sum, (x, y)| sum + x * y)
}

fn score_part_2((p1, p2): (Player, Player)) -> usize {
    let (p1, p2) = (universe_counts(p1), universe_counts(p2));

    // In order to get the number of 'universes' where a player won at a given turn
    // we need to multiply the not_won counts for the loser by the won counts for the winner,
    // since all the rolls are independent events.
    // We sum this product to get the total 'universe' count.

    // p1 wins one turn ahead of p2 since they start first
    let p1_winning_total = sum_product(
        p1.num_won_by_turn.into_iter().skip(1),
        p2.num_not_won_by_turn.into_iter(),
    );
    // p2 wins on the same turn as p1
    let p2_winning_total = sum_product(
        p1.num_won_by_turn.into_iter(),
        p2.num_not_won_by_turn.into_iter(),
    );

    p1_winning_total.max(p2_winning_total)
}

#[aoc(day21, part1)]
pub fn part_1(input: &str) -> usize {
    score_part_1(parse(input))
}

#[aoc(day21, part2)]
pub fn part_2(input: &str) -> usize {
    score_part_2(parse(input))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let input = "\
Player 1 starting position: 4
Player 2 starting position: 8
";
        assert_eq!(739785, part_1(input));
        assert_eq!(444356092776315, part_2(input));
    }
}

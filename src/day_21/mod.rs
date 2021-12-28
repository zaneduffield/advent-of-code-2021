use std::mem;

use rustc_hash::FxHashMap;

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
        self.pos = (self.pos + n) % 10;
        self.score += self.pos + 1;
    }
}

fn parse(input: &str) -> (Player, Player) {
    let mut lines = input.lines();
    let p1 = lines.next().unwrap().bytes().last().unwrap() - b'0';
    let p2 = lines.next().unwrap().bytes().last().unwrap() - b'0';
    (Player::new(p1 as usize - 1), Player::new(p2 as usize - 1))
}

const ROLLS_PER_TURN: usize = 3;
fn score_part_1((mut p1, mut p2): (Player, Player)) -> usize {
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    players: (Player, Player),
    partial_roll: usize,
    partial_rolls_remaining: usize,
}
struct WinningUniversesCache {
    cache: FxHashMap<State, (usize, usize)>,
}

impl WinningUniversesCache {
    fn new() -> WinningUniversesCache {
        WinningUniversesCache {
            cache: FxHashMap::default(),
        }
    }
    fn get(&mut self, args: State) -> (usize, usize) {
        match self.cache.get(&args) {
            Some(val) => *val,
            None => {
                let val = num_winning_universes(self, args);
                self.cache.insert(args, val);
                val
            }
        }
    }
}

fn num_winning_universes(cache: &mut WinningUniversesCache, state: State) -> (usize, usize) {
    let State {
        players: (p1, p2),
        partial_roll,
        partial_rolls_remaining,
    } = state;

    const WINNING_SCORE: usize = 21;
    let (mut p1_count, mut p2_count) = (0, 0);
    for roll in [1, 2, 3] {
        let roll = roll + partial_roll;
        let counts = if partial_rolls_remaining == 1 {
            let mut p1 = p1;
            p1.advance(roll);
            if p1.score >= WINNING_SCORE {
                (0, 1)
            } else {
                cache.get(State {
                    players: (p2, p1),
                    partial_roll: 0,
                    partial_rolls_remaining: ROLLS_PER_TURN,
                })
            }
        } else {
            cache.get(State {
                players: (p2, p1),
                partial_roll: roll,
                partial_rolls_remaining: partial_rolls_remaining - 1,
            })
        };
        p1_count += counts.1;
        p2_count += counts.0;
    }

    (p1_count, p2_count)
}

fn score_part_2((p1, p2): (Player, Player)) -> usize {
    let mut cache = WinningUniversesCache::new();
    let (p1_score, p2_score) = cache.get(State {
        players: (p1, p2),
        partial_roll: 0,
        partial_rolls_remaining: ROLLS_PER_TURN,
    });
    p1_score.max(p2_score)
}

struct DeterministicDie {
    n: usize,
    max: usize,
}

impl Iterator for DeterministicDie {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.n = (self.n + 1) % self.max;
        Some(self.n)
    }
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

use itertools::Itertools;
use rustc_hash::FxHashMap;

type Id = usize;
type Pair = (Id, Id);
struct Rule {
    from: Pair,
    to: Id,
}

struct Poly {
    template: Vec<Id>,
    mapping: Vec<Option<Id>>,
    max_id: Id,
    pair_counts: Vec<u64>,
}

impl Poly {
    fn new(template: Vec<Id>, rules: Vec<Rule>) -> Poly {
        let max_id = rules.iter().fold(0, |max, r| {
            *[r.from.0, r.from.1, r.to, max].iter().max().unwrap_or(&max)
        }) as Id
            + 1;

        let mut poly = Poly {
            template: template.clone(),
            max_id,
            mapping: vec![None; max_id * max_id],
            pair_counts: vec![0; max_id * max_id],
        };

        for r in rules {
            let idx = poly.pair_idx(r.from);
            poly.mapping[idx] = Some(r.to);
        }

        for p in template.windows(2) {
            poly.inc_pair((p[0], p[1]), 1)
        }

        poly
    }

    fn pair_idx(&self, pair: Pair) -> Id {
        let pair = (pair.0, pair.1);
        pair.0 * self.max_id + pair.1
    }

    fn pair_count(&self, pair: Pair) -> u64 {
        let idx = self.pair_idx(pair);
        self.pair_counts[idx]
    }

    fn inc_pair(&mut self, pair: Pair, inc: u64) {
        let idx = self.pair_idx(pair);
        self.pair_counts[idx] += inc;
    }

    fn expand(&self, pair: Pair) -> Option<Id> {
        self.mapping[self.pair_idx(pair)]
    }

    fn generation(&mut self) {
        let mut new_counts = vec![0; self.max_id * self.max_id];
        for i in 0..self.max_id {
            for j in 0..self.max_id {
                let pair = (i, j);
                if let Some(to) = self.expand(pair) {
                    let count = self.pair_count(pair);
                    new_counts[self.pair_idx((i, to))] += count;
                    new_counts[self.pair_idx((to, j))] += count;
                }
            }
        }
        self.pair_counts = new_counts;
    }

    fn solve(&mut self, n_iters: u32) -> u64 {
        (0..n_iters).for_each(|_| self.generation());

        let mut counts = vec![0; self.max_id];
        for i in 0..self.max_id {
            for j in 0..self.max_id {
                let pair_count = self.pair_count((i, j));
                counts[i] += pair_count;
                counts[j] += pair_count;
            }
        }
        counts[*self.template.first().unwrap()] += 1;
        counts[*self.template.last().unwrap()] += 1;

        let max = counts.iter().max().unwrap() / 2;
        let min = counts.iter().min().unwrap() / 2;
        max - min
    }
}

fn parse(input: &str) -> Poly {
    let mut idx_map = FxHashMap::default();
    let mut next_id = 0;
    let mut get_id = |b: u8, idx_map: &mut FxHashMap<u8, Id>| match idx_map.get(&b) {
        Some(id) => *id,
        None => {
            idx_map.insert(b, next_id);
            next_id += 1;
            next_id - 1
        }
    };

    let mut lines = input.lines();
    let template = lines
        .next()
        .unwrap()
        .bytes()
        .map(|b| get_id(b, &mut idx_map))
        .collect();
    lines.next();

    let rules = lines
        .map(|line| {
            line.replace(" -> ", "")
                .bytes()
                .map(|b| get_id(b, &mut idx_map))
                .collect_tuple()
                .unwrap()
        })
        .map(|(a, b, to)| Rule { from: (a, b), to })
        .collect();

    Poly::new(template, rules)
}

#[aoc(day14, part1)]
pub fn part_1(input: &str) -> u64 {
    let mut poly = parse(input);
    poly.solve(10)
}

#[aoc(day14, part2)]
pub fn part_2(input: &str) -> u64 {
    let mut poly = parse(input);
    poly.solve(40)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        assert_eq!(1588, part_1(input));
        assert_eq!(2188189693529, part_2(input));
    }
}

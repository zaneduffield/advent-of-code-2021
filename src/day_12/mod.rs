use std::vec;

use rustc_hash::FxHashMap;

const SIZE: usize = 15;
type Graph = Vec<Vec<Cave>>;

const START_NAME: &str = "start";
const END_NAME: &str = "end";
const END_ID: u8 = 0;
const START_ID: u8 = 1;
const START: Cave = Cave::Small(START_ID);
const END: Cave = Cave::Small(END_ID);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Cave {
    Big(u8),
    Small(u8),
}
use Cave::*;

impl Cave {
    fn idx(self) -> usize {
        match self {
            Small(v) | Big(v) => v as usize,
        }
    }
}

#[derive(Debug, Clone)]
struct VisitTracker {
    counts: Vec<u8>,
    can_visit_twice: bool,
}

impl VisitTracker {
    fn new(can_visit_twice: bool, size: usize) -> Self {
        let mut counts = vec![0; size];
        counts[END.idx()] = 2;
        counts[START.idx()] = 2;
        Self {
            counts,
            can_visit_twice,
        }
    }

    fn can_visit(&self, s: Cave) -> bool {
        let c = self.counts[s.idx()];
        c == 0 || (c == 1 && self.can_visit_twice)
    }

    fn add(&mut self, s: Cave) {
        self.counts[s.idx()] += 1;
        self.can_visit_twice = self.can_visit_twice && self.counts[s.idx()] < 2;
    }
}

fn num_paths(graph: &[Vec<Cave>], start: Cave, tracker: &VisitTracker) -> usize {
    graph[start.idx()]
        .iter()
        .map(|&n| {
            if n == END {
                1
            } else if tracker.can_visit(n) {
                if matches!(n, Small(_)) {
                    let mut new_tracker = tracker.clone();
                    new_tracker.add(n);
                    num_paths(graph, n, &new_tracker)
                } else {
                    num_paths(graph, n, tracker)
                }
            } else {
                0
            }
        })
        .sum()
}

#[inline]
fn add_cave<'a>(cave_by_name: &mut FxHashMap<&'a str, Cave>, name: &'a str, last_id: &mut u8) {
    cave_by_name.entry(name).or_insert_with(|| {
        *last_id += 1;
        if name.chars().all(char::is_lowercase) {
            Small(*last_id)
        } else {
            Big(*last_id)
        }
    });
}

fn parse(input: &str) -> Graph {
    // TODO make this capable of processing graphs of any size
    let mut graph = vec![Vec::new(); SIZE];
    let mut cave_by_name = FxHashMap::default();
    cave_by_name.insert(START_NAME, START);
    cave_by_name.insert(END_NAME, END);
    let mut last_id = std::cmp::max(START_ID, END_ID);
    for line in input.lines() {
        let (src, dest) = line.split_once('-').unwrap();
        add_cave(&mut cave_by_name, src, &mut last_id);
        add_cave(&mut cave_by_name, dest, &mut last_id);

        let (src, dest) = (cave_by_name[&src], cave_by_name[&dest]);
        graph[src.idx()].push(dest);
        graph[dest.idx()].push(src);
    }

    graph
}

#[aoc(day12, part1)]
pub fn part_1(input: &str) -> usize {
    let graph = parse(input);
    num_paths(&graph, START, &VisitTracker::new(false, graph.len()))
}

#[aoc(day12, part2)]
pub fn part_2(input: &str) -> usize {
    let graph = parse(input);
    num_paths(&graph, START, &VisitTracker::new(true, graph.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        assert_eq!(10, part_1(input));
        assert_eq!(36, part_2(input));
    }
}

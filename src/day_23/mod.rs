use std::{cmp::Reverse, collections::BinaryHeap, fmt::Debug, mem};

use itertools::Itertools;
use rustc_hash::FxHashMap;

const NUM_PODS: usize = 4;
const HALL_LEN: usize = 11;
const HALL_END_BUFF: usize = 2;
const POD_GAP: usize = 2;

const EMPTY: u8 = 0;

fn abs_diff<T: std::ops::Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Graph<const POD_LEN: usize> {
    pods: [[u8; POD_LEN]; NUM_PODS],
    hallway: [u8; HALL_LEN],
    cost: u64,
}

fn hallway_pos_for_pod(pod: usize) -> usize {
    HALL_END_BUFF + pod * POD_GAP
}

fn pos_not_in_front_of_pod(pos: usize) -> bool {
    !(HALL_END_BUFF..HALL_LEN - HALL_END_BUFF).contains(&pos)
        || ((pos - HALL_END_BUFF) % POD_GAP != 0)
}

fn cost_per_step_by_idx(i: usize) -> u64 {
    [1, 10, 100, 1000][i]
}

fn cost_per_step(b: u8) -> u64 {
    cost_per_step_by_idx(target_pod_idx(b))
}

fn target_pod_idx(b: u8) -> usize {
    (b - b'A') as usize
}

impl<const POD_LEN: usize> Graph<POD_LEN> {
    fn is_solved(&self) -> bool {
        (b'A'..=b'D')
            .zip(self.pods.iter())
            .all(|(b, pod)| pod.iter().all(|&o| b == o))
    }

    // assuming the pods can move straight through one another, giving an admissible heuristic
    fn heuristic(&self) -> u64 {
        let mut move_into_pod_counts = [0; NUM_PODS];
        // move directly from pod to pod
        let from_pod_cost = self.pods.iter().enumerate().fold(0, |sum, (pod_idx, pod)| {
            pod.iter()
                .enumerate()
                .filter(|(_, b)| **b != EMPTY)
                .fold(sum, |sum, (pos, &b)| {
                    let target_idx = target_pod_idx(b);
                    if pod_idx == target_idx {
                        sum
                    } else {
                        let dist_out_of_pod = pos + 1;
                        move_into_pod_counts[target_idx] += 1;
                        let dist = dist_out_of_pod + abs_diff(target_idx, pod_idx) * POD_GAP;
                        sum + cost_per_step(b) * dist as u64
                    }
                })
        });

        // move from hall to pod
        let from_hall_cost = self
            .hallway
            .iter()
            .enumerate()
            .filter(|(_, b)| **b != EMPTY)
            .fold(0, |sum, (pos, &b)| {
                let target_idx = target_pod_idx(b);
                move_into_pod_counts[target_idx] += 1;
                sum + cost_per_step(b) * abs_diff(pos, hallway_pos_for_pod(target_idx)) as u64
            });

        let pod_fill_cost = move_into_pod_counts
            .iter()
            .enumerate()
            .fold(0, |sum, (i, count)| {
                sum + cost_per_step_by_idx(i) * count * (count + 1) / 2
            });

        from_pod_cost + from_hall_cost + pod_fill_cost
    }

    fn swap_pod_with_hallway(
        &self,
        pod_idx: usize,
        pod_pos: usize,
        hallway_pos: usize,
    ) -> Graph<POD_LEN> {
        let mut out = self.clone();
        mem::swap(
            &mut out.pods[pod_idx][pod_pos],
            &mut out.hallway[hallway_pos],
        );
        let moved_val = match out.pods[pod_idx][pod_pos] {
            EMPTY => out.hallway[hallway_pos],
            b => b,
        };
        out.cost += cost_per_step(moved_val)
            * (1 + pod_pos + abs_diff(hallway_pos, hallway_pos_for_pod(pod_idx))) as u64;
        out
    }

    fn pod_target_pos(&self, pod_idx: usize) -> Option<usize> {
        (0..POD_LEN)
            .rev()
            .find(|&pod_pos| self.pods[pod_idx][pod_pos] == EMPTY)
    }

    fn push_all_hallway_moves<I: Iterator<Item = usize>>(
        &self,
        range: I,
        pod_idx: usize,
        pod_pos: usize,
        out: &mut Vec<Graph<POD_LEN>>,
    ) {
        out.extend(
            range
                .filter(|&h_pos| pos_not_in_front_of_pod(h_pos))
                .take_while(|&h_pos| self.hallway[h_pos] == EMPTY)
                .map(|h_pos| self.swap_pod_with_hallway(pod_idx, pod_pos, h_pos)),
        );
    }

    fn neighbours(&self) -> Vec<Graph<POD_LEN>> {
        let mut out = vec![];
        // moves from pods to hallway
        for (pod_idx, pod) in self.pods.iter().enumerate() {
            if let Some(pod_pos) = pod.iter().position(|b| *b != EMPTY) {
                let h_pos = hallway_pos_for_pod(pod_idx);
                self.push_all_hallway_moves(h_pos + 1..HALL_LEN, pod_idx, pod_pos, &mut out);
                self.push_all_hallway_moves((0..h_pos).rev(), pod_idx, pod_pos, &mut out);
            }
        }

        // moves from hallway to pods
        for (pos, &val) in self
            .hallway
            .iter()
            .enumerate()
            .filter(|(_, b)| **b != EMPTY)
        {
            let pod_idx = target_pod_idx(val);
            // if the pod contains anything different, don't move there
            if self.pods[pod_idx].iter().any(|&b| b != EMPTY && b != val) {
                continue;
            }
            let target_pos = hallway_pos_for_pod(pod_idx);
            let hall_range = if target_pos > pos {
                pos + 1..target_pos + 1
            } else {
                target_pos..pos
            };
            if self.hallway[hall_range].iter().all(|b| *b == EMPTY) {
                if let Some(pod_pos) = self.pod_target_pos(pod_idx) {
                    out.push(self.swap_pod_with_hallway(pod_idx, pod_pos, pos));
                }
            }
        }

        out
    }

    fn astar_min_cost(&self) -> Option<u64> {
        let mut min_costs = FxHashMap::default();
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(self.heuristic()), self.clone()));

        loop {
            let (_, graph) = match heap.pop() {
                None => return None,
                Some(next) => next,
            };
            if graph.is_solved() {
                return Some(graph.cost);
            }
            let ns = graph.neighbours();
            for n in ns {
                let state = (n.pods, n.hallway);
                if &n.cost < min_costs.get(&state).unwrap_or(&u64::MAX) {
                    min_costs.insert(state, n.cost);
                    heap.push((Reverse(n.cost + n.heuristic()), n));
                }
            }
        }
    }
}

fn parse<const POD_LEN: usize>(input: &str) -> Graph<POD_LEN> {
    let lines = input.lines().skip(2);
    let mut pods = [[EMPTY; POD_LEN]; NUM_PODS];
    for (i, line) in lines.take(POD_LEN).enumerate() {
        for (n, pod) in pods.iter_mut().enumerate() {
            pod[i] = *line.as_bytes().get(1 + hallway_pos_for_pod(n)).unwrap();
        }
    }
    Graph {
        pods,
        hallway: [EMPTY; HALL_LEN],
        cost: 0,
    }
}

#[allow(unstable_name_collisions)]
impl<const POD_LEN: usize> Debug for Graph<POD_LEN> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let byte_to_char = |b: u8| if b == EMPTY { '.' } else { b as char };
        f.write_str("#############\n#").unwrap();
        f.write_str(
            &self
                .hallway
                .into_iter()
                .map(byte_to_char)
                .collect::<String>(),
        )
        .unwrap();
        f.write_str("#\n").unwrap();

        for i in 0..POD_LEN {
            f.write_fmt(format_args!(
                "###{}###\n",
                self.pods
                    .iter()
                    .map(|p| byte_to_char(p[i]))
                    .intersperse_with(|| '#')
                    .collect::<String>(),
            ))
            .unwrap();
        }
        f.write_str("  #########  \n")
    }
}

#[aoc(day23, part1)]
pub fn part_1(input: &str) -> u64 {
    parse::<2>(input).astar_min_cost().unwrap()
}

#[aoc(day23, part2)]
pub fn part_2(input: &str) -> u64 {
    let mut input = input.to_owned();
    input.insert_str(42, "  #D#C#B#A#\n  #D#B#A#C#\n");
    parse::<4>(&input).astar_min_cost().unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let input = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";
        assert_eq!(12521, part_1(input));
        assert_eq!(44169, part_2(input));
    }
}

use std::collections::{BTreeMap, BTreeSet};

const START: &str = "start";
const END: &str = "end";

struct Vertex<'a> {
    name: &'a str,
    is_small: bool,
    nbours: BTreeSet<&'a str>,
}

impl<'a> Vertex<'a> {
    fn new(name: &str) -> Vertex {
        Vertex {
            name,
            is_small: name.chars().all(char::is_lowercase),
            nbours: BTreeSet::new(),
        }
    }
}

type VertexByName<'a> = BTreeMap<&'a str, Vertex<'a>>;

fn insert_pair<'a>(from: &'a str, to: &'a str, vertex_map: &mut VertexByName<'a>) {
    vertex_map
        .entry(to)
        .or_insert_with(|| Vertex::new(to))
        .nbours
        .insert(from);
}

fn parse(input: &str) -> VertexByName {
    let mut vertex_map = VertexByName::new();

    for line in input.lines() {
        let (src, dest) = line.split_once('-').unwrap();
        insert_pair(src, dest, &mut vertex_map);
        insert_pair(dest, src, &mut vertex_map);
    }

    vertex_map
}

fn num_paths<'a>(
    start: &'a Vertex,
    end: &Vertex,
    map: &VertexByName,
    visited: &mut BTreeSet<&'a str>,
    mut have_double_visited: bool,
) -> u32 {
    if start.is_small && have_double_visited && visited.contains(start.name)
        || (start.name == START || start.name == END) && visited.contains(start.name)
    {
        0
    } else if std::ptr::eq(start, end) {
        1
    } else {
        if start.is_small {
            have_double_visited = !visited.insert(start.name) || have_double_visited;
        };

        start
            .nbours
            .iter()
            .map(|&n| map.get(n).unwrap())
            .map(|n| num_paths(n, end, map, &mut visited.clone(), have_double_visited))
            .sum()
    }
}

fn solve(input: &str, double_small_visits_allowed: bool) -> u32 {
    let map = parse(input);
    let start = map.get(START).unwrap();
    let end = map.get(END).unwrap();

    num_paths(
        start,
        end,
        &map,
        &mut BTreeSet::new(),
        !double_small_visits_allowed,
    )
}
#[aoc(day12, part1)]
fn part_1(input: &str) -> u32 {
    solve(input, false)
}

#[aoc(day12, part2)]
fn part_2(input: &str) -> u32 {
    solve(input, true)
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

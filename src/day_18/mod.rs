use std::iter::Peekable;

#[derive(Clone, Copy, Debug)]
struct Entry {
    depth: u8,
    value: u8,
}

fn val(entries: &mut Peekable<impl Iterator<Item = Entry>>, depth: u8) -> u32 {
    let first = entries.peek().unwrap();
    if first.depth == depth {
        entries.next().unwrap().value as u32
    } else {
        recursive_magnitude(entries, depth + 1)
    }
}

fn recursive_magnitude<I>(entries: &mut Peekable<I>, depth: u8) -> u32
where
    I: Iterator<Item = Entry>,
{
    3 * val(entries, depth) + 2 * val(entries, depth)
}

fn magnitude(entries: Vec<Entry>) -> u32 {
    recursive_magnitude(&mut entries.into_iter().peekable(), 1)
}

fn add(a: &[Entry], b: &[Entry]) -> Vec<Entry> {
    let mut out = a.to_vec();
    out.extend_from_slice(b);
    out.iter_mut().for_each(|e| e.depth += 1);
    out
}

fn reduce(mut entries: Vec<Entry>) -> Vec<Entry> {
    loop {
        if !explode(&mut entries) && !split(&mut entries) {
            break;
        }
    }
    entries
}

fn split(entries: &mut Vec<Entry>) -> bool {
    match entries.iter().position(|e| e.value > 9) {
        None => false,
        Some(p) => {
            let old = entries[p].value;
            entries[p].value = old / 2;
            entries[p].depth += 1;
            entries.insert(
                p + 1,
                Entry {
                    depth: entries[p].depth,
                    value: (old + 1) / 2,
                },
            );
            true
        }
    }
}

fn explode(entries: &mut Vec<Entry>) -> bool {
    match entries.iter().position(|e| e.depth > 4) {
        Some(p) if p < entries.len() - 1 => {
            let (left, right) = (entries[p].value, entries[p + 1].value);
            if p > 0 {
                entries[p - 1].value += left;
            }
            if p < entries.len() - 2 {
                entries[p + 2].value += right;
            }
            entries[p].value = 0;
            entries[p].depth -= 1;
            entries.remove(p + 1);
            true
        }
        _ => false,
    }
}

fn parse_num(num: &str) -> Vec<Entry> {
    num.bytes()
        .fold(
            (0, Vec::with_capacity(num.len() / 2)),
            |(mut depth, mut v), value| {
                match value {
                    b'[' => depth += 1,
                    b']' => depth -= 1,
                    b'0'..=b'9' => v.push(Entry {
                        depth,
                        value: value - b'0',
                    }),
                    _ => {}
                }
                (depth, v)
            },
        )
        .1
}

fn parse(input: &str) -> Vec<Vec<Entry>> {
    input.lines().map(parse_num).collect()
}

#[aoc(day18, part1)]
fn part_1(input: &str) -> u32 {
    let nums = parse(input);
    let sum = nums.into_iter().reduce(|a, b| reduce(add(&a, &b))).unwrap();
    magnitude(sum)
}

#[aoc(day18, part2)]
fn part_2(input: &str) -> u32 {
    let nums = parse(input);
    (0..nums.len())
        .flat_map(|i| (0..nums.len()).map(move |j| (i, j)))
        .map(|(i, j)| add(&nums[i], &nums[j]))
        .map(reduce)
        .map(magnitude)
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let num = parse_num("[[1,2],[[3,4],5]]");
        assert_eq!(143, magnitude(num));
        let num = parse_num("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        assert_eq!(1137, magnitude(num));

        let input = "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";
        assert_eq!(4140, part_1(input));
        assert_eq!(3993, part_2(input));
    }
}

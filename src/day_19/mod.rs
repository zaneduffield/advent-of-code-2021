use std::hash::Hash;

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
struct Vector(i16, i16, i16);

impl From<(i16, i16, i16)> for Vector {
    fn from(t: (i16, i16, i16)) -> Self {
        Vector(t.0, t.1, t.2)
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Coord {
    X,
    Y,
    Z,
}

impl Coord {
    fn apply(self, v: &Vector) -> i16 {
        match self {
            Coord::X => v.0,
            Coord::Y => v.1,
            Coord::Z => v.2,
        }
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Sign {
    Neg(Coord),
    Pos(Coord),
}

impl Sign {
    fn apply(self, v: &Vector) -> i16 {
        match self {
            Sign::Neg(c) => -c.apply(v),
            Sign::Pos(c) => c.apply(v),
        }
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Rotation(Sign, Sign, Sign);

impl Rotation {
    fn rotations() -> impl Iterator<Item = Self> {
        let r = Rotation;
        let px = Sign::Pos(Coord::X);
        let py = Sign::Pos(Coord::Y);
        let pz = Sign::Pos(Coord::Z);
        let nx = Sign::Neg(Coord::X);
        let ny = Sign::Neg(Coord::Y);
        let nz = Sign::Neg(Coord::Z);
        [
            r(px, py, pz),
            r(nx, py, pz),
            r(px, ny, pz),
            r(px, py, nz),
            r(nx, ny, pz),
            r(nx, py, nz),
            r(px, ny, nz),
            r(nx, ny, nz),
        ]
        .into_iter()
        .flat_map(move |order| {
            let Rotation(x, y, z) = order;
            [
                r(x, y, z),
                r(x, z, y),
                r(y, x, z),
                r(y, z, x),
                r(z, x, y),
                r(z, y, x),
            ]
            .into_iter()
        })
    }
    fn apply(self, v: &Vector) -> Vector {
        Vector(self.0.apply(v), self.1.apply(v), self.2.apply(v))
    }
}

impl Vector {
    fn sub(&self, other: &Self) -> Self {
        Vector(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
    fn add(&self, other: &Self) -> Self {
        Vector(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
    fn manhattan_distance(&self, other: &Self) -> u32 {
        (self.0 - other.0).abs() as u32
            + (self.1 - other.1).abs() as u32
            + (self.2 - other.2).abs() as u32
    }
}

#[derive(Clone)]
struct Scan {
    beacons: Vec<Vector>,
}

fn parse(input: &str) -> Vec<Scan> {
    input
        .split("\n\n")
        .map(|section| Scan {
            beacons: section
                .lines()
                .skip(1)
                .map(|line| {
                    line.split(',')
                        .map(|s| s.parse().unwrap())
                        .collect_tuple::<(i16, i16, i16)>()
                        .unwrap()
                        .into()
                })
                .collect(),
        })
        .collect()
}

fn do_stuff(
    unoriented_scans: &mut Vec<Scan>,
    unified_beacons: &mut FxHashSet<Vector>,
) -> Option<Vector> {
    let mut diffs = FxHashMap::default();
    for (i, scan) in unoriented_scans.iter().enumerate() {
        for r in Rotation::rotations() {
            diffs.clear();
            let rotated_beacons: Vec<_> = scan.beacons.iter().map(|b| r.apply(b)).collect();
            for ref_b in unified_beacons.iter() {
                for other_b in &rotated_beacons {
                    let diff = ref_b.sub(other_b);
                    let diff_count = diffs.entry(diff).or_insert(0);
                    *diff_count += 1;
                    if *diff_count >= 12 {
                        let translated_rotated_beacons = &rotated_beacons
                            .iter()
                            .map(|b| b.add(&diff))
                            .collect::<Vec<_>>();
                        unified_beacons.extend(translated_rotated_beacons);
                        unoriented_scans.swap_remove(i);
                        return Some(diff);
                    }
                }
            }
        }
    }
    None
}
#[aoc(day19, part1)]
pub fn part_1(input: &str) -> usize {
    let mut unoriented_scans = parse(input);
    let mut unified_scan = FxHashSet::from_iter(unoriented_scans.swap_remove(0).beacons);

    let mut diffs = vec![];
    while !unoriented_scans.is_empty() {
        diffs.push(do_stuff(&mut unoriented_scans, &mut unified_scan));
    }

    unified_scan.len()
}

#[aoc(day19, part2)]
pub fn part_2(input: &str) -> u32 {
    let mut unoriented_scans = parse(input);
    let mut unified_scan = FxHashSet::from_iter(unoriented_scans.swap_remove(0).beacons);

    let mut diffs = vec![];
    while !unoriented_scans.is_empty() {
        diffs.push(do_stuff(&mut unoriented_scans, &mut unified_scan).unwrap());
    }

    let mut max = 0;
    for a in &diffs {
        for b in &diffs {
            max = a.manhattan_distance(b).max(max);
        }
    }
    max
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let input = "\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
";
        assert_eq!(79, part_1(input));
        assert_eq!(3621, part_2(input));
    }
}

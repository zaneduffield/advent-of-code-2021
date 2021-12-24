use std::time::Instant;

use advent_of_code_2021::*;

macro_rules! run_day {
    ($m:ident, $d:expr, $input: expr) => {
        println!("day {} part 1: {}", $d, $m::part_1($input));
        println!("day {} part 2: {}", $d, $m::part_2($input));
        println!();
    };
}

pub fn main() {
    let now = Instant::now();

    run_day!(day_1, "1", include_str!("../../input/2021/day1.txt"));
    run_day!(day_2, "2", include_str!("../../input/2021/day2.txt"));
    run_day!(day_3, "3", include_str!("../../input/2021/day3.txt"));
    run_day!(day_4, "4", include_str!("../../input/2021/day4.txt"));
    run_day!(day_5, "5", include_str!("../../input/2021/day5.txt"));
    run_day!(day_6, "6", include_str!("../../input/2021/day6.txt"));
    run_day!(day_7, "7", include_str!("../../input/2021/day7.txt"));
    run_day!(day_8, "8", include_str!("../../input/2021/day8.txt"));
    run_day!(day_9, "9", include_str!("../../input/2021/day9.txt"));
    run_day!(day_10, "10", include_str!("../../input/2021/day10.txt"));
    run_day!(day_11, "11", include_str!("../../input/2021/day11.txt"));
    run_day!(day_12, "12", include_str!("../../input/2021/day12.txt"));
    run_day!(day_13, "13", include_str!("../../input/2021/day13.txt"));
    run_day!(day_14, "14", include_str!("../../input/2021/day14.txt"));
    run_day!(day_15, "15", include_str!("../../input/2021/day15.txt"));
    run_day!(day_16, "16", include_str!("../../input/2021/day16.txt"));
    run_day!(day_17, "17", include_str!("../../input/2021/day17.txt"));
    run_day!(day_18, "18", include_str!("../../input/2021/day18.txt"));
    run_day!(day_19, "19", include_str!("../../input/2021/day19.txt"));

    println!("Done in {}ms", now.elapsed().as_millis());
}

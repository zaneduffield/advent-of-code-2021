type BoardNum = i64;

use arrayvec::ArrayVec;

#[derive(Clone)]
struct Board<const WIDTH: usize, const HEIGHT: usize> {
    rows: ArrayVec<ArrayVec<(BoardNum, bool), WIDTH>, HEIGHT>,
    last_num: Option<BoardNum>,
    row_tally: [usize; HEIGHT],
    col_tally: [usize; WIDTH],
}

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    fn new(config: &str) -> Board<WIDTH, HEIGHT> {
        Board {
            rows: config
                .lines()
                .map(|s| {
                    s.split_whitespace()
                        .map(|x| (x.parse::<BoardNum>().unwrap(), false))
                        .collect()
                })
                .collect(),
            last_num: None,
            row_tally: [0; HEIGHT],
            col_tally: [0; WIDTH],
        }
    }

    fn score(&self) -> BoardNum {
        self.rows
            .iter()
            .flat_map(|row| row.iter().filter(|x| !x.1).map(|x| x.0))
            .sum::<BoardNum>()
            * self.last_num.unwrap_or(0)
    }

    fn add_num_and_check(&mut self, num: BoardNum) -> bool {
        if self.is_done() {
            return true;
        }

        for (i, row) in self.rows.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                if !col.1 && col.0 == num {
                    col.1 = true;
                    self.row_tally[i] += 1;
                    self.col_tally[j] += 1;
                    if self.row_tally[i] == HEIGHT || self.col_tally[j] == WIDTH {
                        self.last_num = Some(num);
                        return true;
                    }
                }
            }
        }

        false
    }

    fn is_done(&self) -> bool {
        self.last_num.is_some()
    }
}

type ParsedInput = (Vec<BoardNum>, Vec<Board<5, 5>>);

fn parse(contents: &str) -> ParsedInput {
    let (nums, rest) = contents.split_once("\n").unwrap();
    let (_, rest) = rest.split_once("\n").unwrap();
    let boards: Vec<_> = rest.split("\n\n").map(Board::new).collect();
    let nums = nums
        .split(',')
        .map(|n| n.parse::<BoardNum>().unwrap())
        .collect();

    (nums, boards)
}

#[aoc(day4, part1)]
pub fn part_1(input: &str) -> BoardNum {
    let (nums, mut boards) = parse(input);
    for n in nums {
        for board in boards.iter_mut() {
            if board.add_num_and_check(n) {
                return board.score();
            }
        }
    }

    panic!("No winner found!")
}

#[aoc(day4, part2)]
pub fn part_2(input: &str) -> BoardNum {
    let (nums, mut boards) = parse(input);
    for n in nums {
        let mut new_boards = vec![];
        let len = boards.len();
        for mut board in boards {
            if !board.add_num_and_check(n) {
                new_boards.push(board);
            } else if len == 1 {
                return board.score();
            }
        }

        boards = new_boards;
    }

    panic!("No 'last' winner found!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        assert_eq!(part_1(input), 4512);
        assert_eq!(part_2(input), 1924);
    }
}

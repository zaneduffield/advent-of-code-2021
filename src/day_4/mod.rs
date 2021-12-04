pub fn run() {
    let input = include_str!("input.txt");
    println!(
        "day 3\n  part 1: {}\n  part 2: {}\n",
        part_1(input),
        part_2(input)
    );
}

type BoardNum = i64;
struct Board {
    rows: Vec<Vec<(BoardNum, bool)>>,
    last_num: Option<BoardNum>,
}

impl Board {
    fn new(config: &str) -> Board {
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

        let mut bingo = false;
        let mut col_hits = vec![0; self.rows.len()];
        for row in self.rows.iter_mut() {
            let mut row_hits = 0;
            for (i, col) in row.iter_mut().enumerate() {
                col.1 = col.1 || col.0 == num;
                row_hits += col.1 as usize;
                col_hits[i] += col.1 as usize;
            }
            bingo = bingo || row_hits == row.len();
        }
        bingo = bingo || col_hits.iter().any(|x| *x == self.rows.len());

        if bingo {
            self.last_num = Some(num)
        }

        bingo
    }

    fn is_done(&self) -> bool {
        self.last_num.is_some()
    }
}

fn parse(contents: &str) -> (Vec<BoardNum>, Vec<Board>) {
    let (nums, rest) = contents.split_once("\n").unwrap();
    let (_, rest) = rest.split_once("\n").unwrap();
    let boards: Vec<_> = rest.split("\n\n").map(Board::new).collect();
    let nums = nums
        .split(',')
        .map(|n| n.parse::<BoardNum>().unwrap())
        .collect();

    (nums, boards)
}

fn part_1(contents: &str) -> BoardNum {
    let (nums, mut boards) = parse(contents);

    for n in nums {
        for board in boards.iter_mut() {
            if board.add_num_and_check(n) {
                return board.score();
            }
        }
    }

    panic!("No winner found!")
}

fn part_2(contents: &str) -> BoardNum {
    let (nums, mut boards) = parse(contents);
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
    use super::part_1;
    use super::part_2;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
 2  0 12  3  7"
            ),
            4512
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
 2  0 12  3  7"
            ),
            1924
        );
    }
}

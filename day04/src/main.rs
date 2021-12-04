use std::collections::HashMap;

static INPUT: &str = include_str!("../input");

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

fn part_1(input: &'static str) -> u32 {
    let nums = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let mut boards = input
        .split("\n\n")
        .skip(1)
        .filter(|&l| !l.is_empty())
        .map(Board::from)
        .collect::<Vec<_>>();

    for n in nums {
        for b in boards.iter_mut() {
            b.find_and_fill(n);
            if let Some(score) = b.try_final_score(&n) {
                return score;
            }
        }
    }

    todo!();
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct XY(usize, usize);

#[derive(Debug)]
struct BingoNum(XY, u32, bool);

#[derive(Debug)]
struct Board(HashMap<XY, BingoNum>);

impl From<&str> for Board {
    fn from(raw: &str) -> Self {
        let numbers = raw
            .lines()
            .enumerate()
            .flat_map(|(x, row)| {
                row.split_ascii_whitespace()
                    .filter_map(|n| n.parse::<u32>().ok())
                    .enumerate()
                    .map(|(y, num)| (XY(x, y), BingoNum(XY(x, y), num, false)))
                    .collect::<Vec<_>>()
            })
            .collect();

        Self(numbers)
    }
}

impl Board {
    fn find_and_fill(&mut self, find_val: u32) -> bool {
        let found = self
            .0
            .values_mut()
            .find(|BingoNum(_, val, _)| find_val == *val);

        if let Some(BingoNum(_, _, drawn)) = found {
            *drawn = true;
            true
        } else {
            false
        }
    }

    fn as_rows_and_cols(&self) -> (Vec<Vec<&BingoNum>>, Vec<Vec<&BingoNum>>) {
        let mut rows = vec![Vec::new(); 5];
        let mut cols = vec![Vec::new(); 5];

        for (x, row) in rows.iter_mut().enumerate().take(5) {
            for (y, col) in cols.iter_mut().enumerate().take(5) {
                let curr = self.0.get(&XY(x, y)).unwrap();
                row.push(curr);
                col.push(curr);
            }
        }

        (rows, cols)
    }

    fn try_final_score(&self, last_called: &u32) -> Option<u32> {
        let (rows, cols) = self.as_rows_and_cols();

        for r in [rows, cols].concat() {
            if r.iter().filter(|BingoNum(_, _, drawn)| *drawn).count() == 5 {
                let board_sum = self
                    .0
                    .values()
                    .filter(|BingoNum(_, _, drawn)| !*drawn)
                    .map(|BingoNum(_, val, _)| val)
                    .sum::<u32>();

                return Some(board_sum * last_called);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 4512);
    }
}

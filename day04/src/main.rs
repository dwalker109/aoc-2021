use std::collections::HashMap;

static INPUT: &str = include_str!("../input");

fn main() {
    let results = solve(INPUT);
    println!("Part 1: {}", part_1(&results));
    println!("Part 2: {}", part_2(&results));
}

fn part_1(input: &[u32]) -> &u32 {
    input.first().unwrap()
}

fn part_2(input: &[u32]) -> &u32 {
    input.last().unwrap()
}

fn solve(input: &'static str) -> Vec<u32> {
    let (nums, mut boards) = parse_input(input);
    let mut wins = Vec::new();

    for n in nums {
        for b in boards.iter_mut() {
            b.find_and_fill(n);
            if let Some(score) = b.try_final_score(&n) {
                wins.push((b.id, score));
            }
        }
        boards.retain(|b| !wins.iter().any(|(id, _)| *id == b.id));
    }

    wins.iter().map(|(_, score)| *score).collect()
}

fn parse_input(input: &'static str) -> (Vec<u32>, Vec<Board>) {
    let nums = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let boards = input
        .split("\n\n")
        .skip(1)
        .filter(|&l| !l.is_empty())
        .map(Board::from)
        .collect::<Vec<_>>();

    (nums, boards)
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct XY(usize, usize);

#[derive(Debug)]
struct BingoNum(XY, u32, bool);

#[derive(Debug)]
struct Board {
    id: uuid::Uuid,
    data: HashMap<XY, BingoNum>,
}

impl From<&str> for Board {
    fn from(raw: &str) -> Self {
        let data = raw
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

        Self {
            id: uuid::Uuid::new_v4(),
            data,
        }
    }
}

impl Board {
    fn find_and_fill(&mut self, find_val: u32) -> bool {
        let found = self
            .data
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
        let mut rows = vec![Vec::with_capacity(5); 5];
        let mut cols = vec![Vec::with_capacity(5); 5];

        for (x, row) in rows.iter_mut().enumerate().take(5) {
            for (y, col) in cols.iter_mut().enumerate().take(5) {
                let curr = self.data.get(&XY(x, y)).unwrap();
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
                    .data
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
        let r = super::solve(INPUT);
        assert_eq!(super::part_1(&r), &4512);
    }

    #[test]
    fn part_2() {
        let r = super::solve(INPUT);
        assert_eq!(super::part_2(&r), &1924);
    }
}

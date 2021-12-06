use std::{cell::RefCell, collections::HashMap, rc::Rc};

static INPUT: &str = include_str!("../input");

// Input data always conforms to this
const MAX_ROW_COL: usize = 5;

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
            b.find_and_fill(&n);
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct XY(usize, usize);

#[derive(Debug)]
struct BingoNum {
    xy: XY,
    val: u32,
    drawn: bool,
}

impl BingoNum {
    fn new(x: usize, y: usize, val: u32) -> RcBingoNum {
        Rc::new(RefCell::new(Self {
            xy: XY(x, y),
            val,
            drawn: false,
        }))
    }
}

type RcBingoNum = Rc<RefCell<BingoNum>>;

#[derive(Debug)]
struct Board {
    id: uuid::Uuid,
    data: Vec<RcBingoNum>,
    by_xy: HashMap<XY, RcBingoNum>,
    by_val: HashMap<u32, RcBingoNum>,
    rows: Vec<Vec<RcBingoNum>>,
    cols: Vec<Vec<RcBingoNum>>,
}

impl From<&str> for Board {
    fn from(raw: &str) -> Self {
        // Raw data, used to build other views and for summing
        let data = raw
            .lines()
            .enumerate()
            .flat_map(|(x, row)| {
                row.split_ascii_whitespace()
                    .filter_map(|n| n.parse::<u32>().ok())
                    .enumerate()
                    .map(|(y, num)| BingoNum::new(x, y, num))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // Access data by coordinates
        let by_xy = data
            .iter()
            .map(|bn| ((*bn).borrow().xy, Rc::clone(bn)))
            .collect::<HashMap<_, _>>();

        // Access data by value
        let by_val = data
            .iter()
            .map(|bn| ((*bn).borrow().val, Rc::clone(bn)))
            .collect::<HashMap<_, _>>();

        // Allow traversal of data grouped by rows & cols
        let mut rows = vec![Vec::with_capacity(MAX_ROW_COL); MAX_ROW_COL];
        let mut cols = vec![Vec::with_capacity(MAX_ROW_COL); MAX_ROW_COL];
        for (x, row) in rows.iter_mut().enumerate().take(MAX_ROW_COL) {
            for (y, col) in cols.iter_mut().enumerate().take(MAX_ROW_COL) {
                let curr = by_xy.get(&XY(x, y)).unwrap();
                row.push(Rc::clone(curr));
                col.push(Rc::clone(curr));
            }
        }

        Self {
            id: uuid::Uuid::new_v4(),
            data,
            by_xy,
            by_val,
            rows,
            cols,
        }
    }
}

impl Board {
    fn find_and_fill(&mut self, find_val: &u32) -> bool {
        let found = self.by_val.get(find_val);

        if let Some(bn) = found {
            bn.borrow_mut().drawn = true;
            true
        } else {
            false
        }
    }

    fn try_final_score(&self, last_called: &u32) -> Option<u32> {
        for r in self.rows.iter().chain(self.cols.iter()) {
            if r.iter().filter(|bn| (**bn).borrow().drawn).count() == MAX_ROW_COL {
                let board_sum = self
                    .data
                    .iter()
                    .filter(|&bn| !(*bn).borrow().drawn)
                    .map(|bn| (*bn).borrow().val)
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

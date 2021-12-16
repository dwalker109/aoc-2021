use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[logging_timer::time]
fn part_1(input: &str) -> usize {
    solve(input)
}

#[logging_timer::time]
fn part_2(input: &str) -> usize {
    solve(&expanded_input(input))
}

fn solve(input: &str) -> usize {
    let mut spt = SpTree::from(input);

    while spt.visited.len() < spt.vertices.len() {
        let Reverse(WorkItem(curr_cost, curr_xy)) = spt.work_queue.pop().unwrap();

        for xy in curr_xy
            .adjacent()
            .iter()
            .filter(|v| !spt.visited.contains(v))
        {
            if let Some(next) = spt.vertices.get_mut(xy) {
                let cost = curr_cost.unwrap() + next.risk as usize;

                if next.cost.is_inf() || cost < *next.cost.unwrap() {
                    next.cost = Cost::Val(cost);
                    next.prev = Some(curr_xy);

                    spt.work_queue
                        .push(Reverse(WorkItem(Cost::Val(cost), next.xy)));
                }
            }
        }

        spt.visited.insert(curr_xy);
    }

    *spt.vertices.get(&spt.target).unwrap().cost.unwrap()
}

#[logging_timer::time]
fn expanded_input(input: &str) -> String {
    let exp_right = input.lines().map(|l| {
        let as_digits = l.chars().filter_map(|c| c.to_digit(10));

        vec![as_digits.clone(); 5]
            .into_iter()
            .enumerate()
            .flat_map(|(i, c)| c.map(move |c| (c + (i as u32) - 1) % 9 + 1))
    });

    let exp_full = exp_right.map(|l| {
        vec![l.clone(); 5]
            .into_iter()
            .enumerate()
            .map(|(i, c)| c.map(move |c| (c + (i as u32) - 1) % 9 + 1).join(""))
    });

    let reordered =
        (0..5).flat_map(|i| exp_full.clone().flatten().skip(i).step_by(5).collect_vec());

    reordered.collect_vec().join("\n")
}

struct SpTree {
    vertices: HashMap<XY, Vertex>,
    visited: HashSet<XY>,
    work_queue: BinaryHeap<Reverse<WorkItem>>,
    target: XY,
}

impl From<&str> for SpTree {
    fn from(raw: &str) -> Self {
        let mut vertices = raw
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .filter_map(|c| c.to_digit(10))
                    .enumerate()
                    .map(move |(x, r)| (XY(x as isize, y as isize), Vertex::from(((x, y), r))))
            })
            .collect::<HashMap<_, _>>();

        let ((&start, _), (&target, _)) = vertices
            .iter()
            .minmax_by_key(|(&xy, _)| xy)
            .into_option()
            .unwrap();

        vertices.get_mut(&start).unwrap().cost = Cost::Val(0);

        let mut work_queue = BinaryHeap::new();
        work_queue.push(Reverse(WorkItem(Cost::Val(0), start)));

        let visited = HashSet::with_capacity(vertices.len());

        Self {
            vertices,
            work_queue,
            visited,
            target,
        }
    }
}

struct Vertex {
    xy: XY,
    risk: u8,
    cost: Cost,
    prev: Option<XY>,
}

impl From<((usize, usize), u32)> for Vertex {
    fn from(((x, y), risk): ((usize, usize), u32)) -> Self {
        Self {
            xy: XY(x as isize, y as isize),
            risk: risk.try_into().unwrap(),
            cost: Cost::Inf,
            prev: None,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct XY(isize, isize);

impl From<(isize, isize)> for XY {
    fn from((x, y): (isize, isize)) -> Self {
        Self(x, y)
    }
}

impl XY {
    fn adjacent(&self) -> Vec<XY> {
        let XY(x, y) = self;

        vec![XY(x - 1, *y), XY(x + 1, *y), XY(*x, y - 1), XY(*x, y + 1)]
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Cost {
    Inf,
    Val(usize),
}

impl Cost {
    fn is_inf(&self) -> bool {
        matches!(self, Self::Inf)
    }

    fn unwrap(&self) -> &usize {
        match &self {
            Cost::Inf => &usize::MAX,
            Cost::Val(n) => n,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct WorkItem(Cost, XY);

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 40);
    }

    #[test]
    fn part_2() {
        let r = super::part_2(INPUT);
        assert_eq!(r, 315);
    }
}

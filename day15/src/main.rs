use itertools::Itertools;
use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
}

#[logging_timer::time]
fn part_1(input: &str) -> usize {
    let mut spt = SpTree::from(input);

    while spt.visited.len() < spt.vertices.len() {
        let (curr_xy, curr_cost) = {
            let (_, v) = spt
                .vertices
                .iter()
                .sorted_by(|(_, l), (_, r)| {
                    let l_cost = l.cost.unwrap();
                    let r_cost = r.cost.unwrap();

                    Ord::cmp(l_cost, r_cost)
                })
                .find(|(xy, v)| !spt.visited.contains(xy) && !v.cost.is_inf())
                .unwrap();

            (v.xy, *v.cost.unwrap())
        };

        for xy in curr_xy
            .adjacent()
            .iter()
            .filter(|v| !spt.visited.contains(v))
        {
            if let Some(next) = spt.vertices.get_mut(xy) {
                let cost = curr_cost + next.risk as usize;

                if next.cost.is_inf() || cost < *next.cost.unwrap() {
                    next.cost = Cost::Val(cost);
                    next.prev = Some(XY(curr_xy.0, curr_xy.1));
                }
            }
        }

        spt.visited.insert(curr_xy);
    }

    *spt.vertices.get(&spt.finish).unwrap().cost.unwrap()
}

#[derive(Debug)]
struct SpTree {
    vertices: HashMap<XY, Vertex>,
    visited: HashSet<XY>,
    start: XY,
    finish: XY,
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

        let ((&start, _), (&finish, _)) = vertices
            .iter()
            .minmax_by_key(|(&xy, _)| xy)
            .into_option()
            .unwrap();

        vertices.get_mut(&start).unwrap().cost = Cost::Val(0);

        let visited = HashSet::with_capacity(vertices.len());

        Self {
            vertices,
            visited,
            start,
            finish,
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct XY(isize, isize);

impl XY {
    fn adjacent(&self) -> Vec<XY> {
        let XY(x, y) = self;

        vec![XY(x - 1, *y), XY(x + 1, *y), XY(*x, y - 1), XY(*x, y + 1)]
    }
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 40);
    }
}

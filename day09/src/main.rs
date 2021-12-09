use itertools::Itertools;
use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[logging_timer::time]
fn part_1(input: &'static str) -> u32 {
    let heightmap = HeightMap::from(input);

    heightmap
        .low_points()
        .iter()
        .map(|(_, &val)| 1 + val)
        .sum::<u32>()
}

#[logging_timer::time]
fn part_2(input: &'static str) -> u32 {
    let heightmap = HeightMap::from(input);

    heightmap
        .low_points()
        .iter()
        .map(|&low_point| {
            let mut res = HashSet::with_capacity(heightmap.0.len());
            heightmap.explore(None, low_point, &mut res);

            res.len() as u32
        })
        .sorted_by(|a, b| Ord::cmp(b, a))
        .take(3)
        .product::<u32>()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct XY(isize, isize);

#[derive(Clone)]
struct HeightMap(HashMap<XY, u32>);

impl From<&str> for HeightMap {
    fn from(raw: &str) -> Self {
        Self(
            raw.lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars()
                        .enumerate()
                        .map(move |(x, c)| (XY(x as isize, y as isize), c.to_digit(10).unwrap()))
                })
                .collect::<HashMap<_, _>>(),
        )
    }
}

impl HeightMap {
    fn low_points(&self) -> Vec<(&XY, &u32)> {
        self.0
            .iter()
            .filter_map(|(xy, v)| {
                let [(_, u), (_, d), (_, l), (_, r)] = self.adjacent(xy);

                if v < u.unwrap_or(&u32::MAX)
                    && v < d.unwrap_or(&u32::MAX)
                    && v < l.unwrap_or(&u32::MAX)
                    && v < r.unwrap_or(&u32::MAX)
                {
                    Some((xy, v))
                } else {
                    None
                }
            })
            .collect()
    }

    fn adjacent(&self, XY(x, y): &XY) -> [(XY, Option<&u32>); 4] {
        let up = XY(*x, y - 1);
        let down = XY(*x, y + 1);
        let left = XY(x - 1, *y);
        let right = XY(x + 1, *y);

        [
            (up, self.0.get(&up)),
            (down, self.0.get(&down)),
            (left, self.0.get(&left)),
            (right, self.0.get(&right)),
        ]
    }

    fn explore(
        &self,
        prev_xy: Option<&XY>,
        (curr_xy, curr_val): (&XY, &u32),
        res: &mut HashSet<XY>,
    ) {
        res.insert(*curr_xy);

        let [u, d, l, r] = self.adjacent(curr_xy);

        for next in [u, d, l, r]
            .iter()
            .map(|(xy, v)| (xy, v.unwrap_or(&0)))
            .filter(|(&next_xy, &next_val)| {
                Some(&next_xy) != prev_xy && next_val != 9 && next_val > *curr_val
            })
        {
            self.explore(Some(curr_xy), next, res);
        }
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 15);
    }

    #[test]
    fn part_2() {
        let r = super::part_2(INPUT);
        assert_eq!(r, 1134);
    }
}

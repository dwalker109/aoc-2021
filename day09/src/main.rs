use std::collections::HashMap;

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
        .map(|p| 1 + heightmap.0.get(p).unwrap())
        .sum::<u32>()
}

fn part_2(input: &'static str) -> u32 {
    let heightmap = HeightMap::from(input);
    let low_points = heightmap.low_points();

    for lp in low_points {
        let mut size = 1u32;
        let explore = |p: (XY, u32)| {
            let (u, d, l, r) = heightmap.adjacent(&p.0);
            for n in [u, d, l, r].iter().filter(|(_, v)| v.unwrap() > &p.1) {
                size += 1;
                explore(n);
            }
        };
    }

    todo!()
    // let
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct XY(isize, isize);

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
                        .collect::<Vec<_>>()
                })
                .collect::<HashMap<_, _>>(),
        )
    }
}

impl HeightMap {
    fn low_points(&self) -> Vec<&XY> {
        self.0
            .iter()
            .filter_map(|(xy, v)| {
                let ((_, up), (_, down), (_, left), (_, right)) = self.adjacent(xy);

                if v < up.unwrap_or(&u32::MAX)
                    && v < down.unwrap_or(&u32::MAX)
                    && v < left.unwrap_or(&u32::MAX)
                    && v < right.unwrap_or(&u32::MAX)
                {
                    Some(xy)
                } else {
                    None
                }
            })
            .collect()
    }

    fn adjacent(
        &self,
        XY(x, y): &XY,
    ) -> (
        (XY, Option<&u32>),
        (XY, Option<&u32>),
        (XY, Option<&u32>),
        (XY, Option<&u32>),
    ) {
        let up = XY(*x, y - 1);
        let down = XY(*x, y + 1);
        let left = XY(x - 1, *y);
        let right = XY(x + 1, *y);

        (
            (up, self.0.get(&up)),
            (down, self.0.get(&down)),
            (left, self.0.get(&left)),
            (right, self.0.get(&right)),
        )
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

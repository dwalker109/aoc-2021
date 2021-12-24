#![feature(int_abs_diff)]
#![feature(hash_drain_filter)]

use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::HashMap,
};

use itertools::Itertools;

static INPUT: &str = include_str!("../input");

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

fn part_1(input: &str) -> usize {
    let scanners = parse_input(input);

    'outer: while scanners.iter().any(|s| s.normalised.borrow().is_none()) {
        for s1 in scanners.iter().filter(|&s| s.normalised.borrow().is_some()) {
            for s2 in scanners.iter().filter(|&s| s.normalised.borrow().is_none()) {
                for i in 0..24 {
                    let mut aggregated = HashMap::new();

                    for b1 in s1.beacons.iter() {
                        for b2 in s2.beacons.iter() {
                            let b2 = &b2.translate(i);
                            *aggregated
                                .entry((b1.0 - b2.0, b1.1 - b2.1, b1.2 - b2.2))
                                .or_insert(0u8) += 1;
                        }
                    }

                    if let Some((cmp_offset, _)) = aggregated.iter().find(|(_, &qty)| qty >= 12) {
                        dbg!(&cmp_offset);
                        let normalised = s2
                            .beacons
                            .iter()
                            .map(|b2| {
                                Beacon(b2.translate(i).0, b2.translate(i).1, b2.translate(i).2)
                            })
                            .collect::<Vec<_>>();

                        s2.offset.replace(*cmp_offset);
                        s2.overlaps_with.replace(Some(s1.label.clone()));
                        s2.normalised.replace(Some(normalised));

                        continue 'outer;
                    }
                }
            }
        }
    }

    let beacon_map = scanners
        .iter()
        .flat_map(|s| {
            // let offset = s.cumulative_offset(&scanners);

            let mut zero_adjusted = Vec::new();
            let mut overlaps_with = s.overlaps_with.borrow().to_owned();
            let mut offset = s.offset.borrow().to_owned();

            loop {
                zero_adjusted = s
                    .normalised
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|b| Beacon(b.0 + offset.0, b.1 + offset.1, b.2 + offset.2))
                    .collect_vec();

                let next = scanners
                    .iter()
                    .find(|&os| overlaps_with.borrow().as_ref() == Some(&os.label));

                if next.is_none() {
                    break;
                } else {
                    overlaps_with = next.unwrap().overlaps_with.borrow().clone();
                    offset = next.unwrap().offset.borrow().to_owned();
                }
            }

            zero_adjusted
        })
        .sorted()
        .dedup()
        .collect_vec();

    dbg!(&beacon_map);

    beacon_map.len()
}

fn parse_input(input: &str) -> Vec<Scanner> {
    let mut scanners = input.split("\n\n").map(Scanner::from).collect::<Vec<_>>();

    scanners[0].normalised = RefCell::new(Some(scanners[0].beacons.clone()));

    scanners
}

#[derive(Debug)]
struct Scanner {
    label: String,
    offset: RefCell<(i32, i32, i32)>,
    overlaps_with: RefCell<Option<String>>,
    normalised: RefCell<Option<Vec<Beacon>>>,
    beacons: Vec<Beacon>,
}

impl From<&str> for Scanner {
    fn from(raw: &str) -> Self {
        let mut s = raw.lines();

        let label = s.next().unwrap().to_string();
        let beacons = s.map(Beacon::from).collect::<Vec<_>>();

        Self {
            label,
            offset: RefCell::new((0, 0, 0)),
            overlaps_with: RefCell::new(None),
            normalised: RefCell::new(None),
            beacons,
        }
    }
}

impl Scanner {
    fn cumulative_offset(&self, scanners: &[Scanner]) -> (i32, i32, i32) {
        let mut overlaps_with = self.overlaps_with.borrow().to_owned();
        let mut offset = self.offset.borrow().to_owned();

        loop {
            if let Some(next) = scanners
                .iter()
                .find(|&s| Some(s.label.to_owned()) == overlaps_with)
            {
                let (x, y, z) = next.offset.borrow().to_owned();
                offset = (offset.0 + x.abs(), offset.1 + y.abs(), offset.2 + z.abs());
                overlaps_with = next.overlaps_with.borrow().to_owned();
            } else {
                dbg!(&self.label, &self.overlaps_with.borrow().as_ref(), offset);
                return offset;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Beacon(i32, i32, i32);

impl From<&str> for Beacon {
    fn from(raw: &str) -> Self {
        let [x, y, z] = <[i32; 3]>::try_from(
            raw.split(',')
                .filter_map(|b| b.parse::<i32>().ok())
                .collect::<Vec<_>>(),
        )
        .unwrap();

        Self(x, y, z)
    }
}

impl Beacon {
    fn translate(&self, version: u8) -> Beacon {
        let Beacon(x, y, z) = *self;

        match version {
            // positive x
            0 => Beacon(x, y, z),
            1 => Beacon(x, -z, y),
            2 => Beacon(x, -y, -z),
            3 => Beacon(x, z, -y),
            // negative x
            4 => Beacon(-x, -y, z),
            5 => Beacon(-x, z, y),
            6 => Beacon(-x, y, -z),
            7 => Beacon(-x, -z, -y),
            // positive y
            8 => Beacon(y, z, x),
            9 => Beacon(y, -x, z),
            10 => Beacon(y, -z, -x),
            11 => Beacon(y, x, -z),
            // negative y
            12 => Beacon(-y, -z, x),
            13 => Beacon(-y, x, z),
            14 => Beacon(-y, z, -x),
            15 => Beacon(-y, -x, -z),
            // positive z
            16 => Beacon(z, x, y),
            17 => Beacon(z, -y, x),
            18 => Beacon(z, -x, -y),
            19 => Beacon(z, y, -x),
            // negative z
            20 => Beacon(-z, -x, y),
            21 => Beacon(-z, y, x),
            22 => Beacon(-z, x, -y),
            23 => Beacon(-z, -y, -x),
            _ => panic!("only 24 rotations of a cube"),
        }
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 79);
    }
}

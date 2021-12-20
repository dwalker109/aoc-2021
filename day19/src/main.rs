#![feature(int_abs_diff)]

use std::{collections::HashMap, fmt::format};

static INPUT: &str = include_str!("../input");

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

fn part_1(input: &str) -> usize {
    let scanners = parse_input(input);

    dbg!(&scanners);

    while scanners.iter().any(|s| s.normalised.is_none()) {
        for s1 in scanners.iter().filter(|&s| s.normalised.is_some()) {
            for s2 in scanners.iter().filter(|&s| s.normalised.is_none()) {
                for b1 in s1.beacons.iter() {
                    let mut possibles = HashMap::new();
                    for b2 in s2.beacons.iter() {
                        for i in 0..24 {
                            let b2 = &b2.translate(i);
                            if b1 == b2 {
                                *possibles.entry(i).or_insert(0u8) += 1;
                            }
                        }
                    }
                    dbg!(&possibles);
                }
            }
        }
    }

    todo!();
}

fn parse_input(input: &str) -> Vec<Scanner> {
    let mut scanners = input.split("\n\n").map(Scanner::from).collect::<Vec<_>>();

    scanners[0].normalised = Some(scanners[0].beacons.clone());

    scanners
}

#[derive(Debug)]
struct Scanner {
    label: String,
    normalised: Option<Vec<Beacon>>,
    beacons: Vec<Beacon>,
}

impl From<&str> for Scanner {
    fn from(raw: &str) -> Self {
        let mut s = raw.lines();

        let label = s.next().unwrap().to_string();
        let beacons = s.map(Beacon::from).collect::<Vec<_>>();

        Self {
            label,
            normalised: None,
            beacons,
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

impl PartialEq for Beacon {
    fn eq(&self, other: &Self) -> bool {
        let Beacon(x1, y1, z1) = *self;
        let Beacon(x2, y2, z2) = *other;

        // x2 - x1 == y2 - y1
        x2 - x1 == y2 - y1 && x2 - x1 == z2 - z1 && y2 - y1 == z2 - z1
    }
}

impl Beacon {
    fn translate(&self, version: u8) -> Beacon {
        let Beacon(x, y, z) = *self;
        // CHECK THIS!!
        match version {
            0 => Beacon(x, y, z),
            1 => Beacon(x, -z, y),
            2 => Beacon(x, -y, -z),
            3 => Beacon(x, z, -y),
            4 => Beacon(y, z, x),
            5 => Beacon(y, -x, -z),
            6 => Beacon(y, -z, -x),
            7 => Beacon(y, x, -z),
            8 => Beacon(z, x, y),
            9 => Beacon(z, -y, x),
            10 => Beacon(z, -x, -y),
            11 => Beacon(z, y, -x),
            12 => Beacon(-z, -y, -x),
            13 => Beacon(-z, x, -y),
            14 => Beacon(-z, y, x),
            15 => Beacon(-z, -x, y),
            16 => Beacon(-y, -x, -z),
            17 => Beacon(-y, z, -x),
            18 => Beacon(-y, x, z),
            19 => Beacon(-y, -z, x),
            20 => Beacon(-x, -z, -y),
            21 => Beacon(-x, y, -z),
            22 => Beacon(-x, z, y),
            23 => Beacon(-x, -y, z),
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

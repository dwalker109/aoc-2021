use core::panic;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Add};

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[logging_timer::time]
fn part_1(input: &str) -> usize {
    let mut pairs = parse_input(input);

    let initial = pairs.remove(0);
    let res = pairs.into_iter().fold(initial, |acc, cur| acc + cur);

    res.magnitude()
}

#[logging_timer::time]
fn part_2(input: &str) -> usize {
    let pairs = parse_input(input);

    let mut results = Vec::with_capacity(pairs.len() * pairs.len());

    for a in 0..pairs.len() {
        for b in 0..pairs.len() {
            if a == b {
                continue;
            }

            results.push((pairs[a].clone() + pairs[b].clone()).magnitude());
        }
    }

    *results.iter().max().unwrap()
}

fn parse_input(input: &str) -> Vec<Pair> {
    input
        .lines()
        .filter_map(|l| serde_json::from_str::<Pair>(l).ok())
        .collect::<Vec<_>>()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
enum Pair {
    Number(u8),
    Another(Box<Pair>, Box<Pair>),
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pair::Number(n) => write!(f, "{}", n),
            Pair::Another(l, r) => write!(f, "[{},{}]", l, r),
        }
    }
}

impl Add for Pair {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_pair = Self::Another(Box::new(self), Box::new(rhs));

        loop {
            let mut stable = true;

            new_pair.explode(&mut stable, 0);

            if !stable {
                continue;
            }

            new_pair.split(&mut stable);

            if stable {
                break;
            }
        }

        new_pair
    }
}

impl Pair {
    fn unwrap_one(&self) -> u8 {
        if let Self::Number(n) = self {
            return *n;
        }

        panic!("expected a regular number");
    }

    fn unwrap_pair(&self) -> (u8, u8) {
        if let Self::Another(l, r) = self {
            if matches!(**l, Self::Number(_)) && matches!(**r, Self::Number(_)) {
                return (l.unwrap_one(), r.unwrap_one());
            }
        }

        panic!("expected a pair of regular numbers");
    }

    fn magnitude(&self) -> usize {
        match self {
            Pair::Number(n) => *n as usize,
            Pair::Another(l, r) => {
                if matches!(**l, Pair::Number(_)) && matches!(**r, Pair::Number(_)) {
                    let (l, r) = self.unwrap_pair();

                    3 * (l as usize) + 2 * (r as usize)
                } else {
                    let l = l.magnitude();
                    let r = r.magnitude();

                    3 * (l as usize) + 2 * (r as usize)
                }
            }
        }
    }

    fn explode(
        &mut self,
        stable: &mut bool,
        lvl: u8,
    ) -> (Option<ExplodeCarry>, Option<ExplodeCarry>) {
        if !*stable {
            return (None, None);
        }

        if matches!(self, Self::Number(_)) {
            return (None, None);
        }

        if lvl == 4 {
            let (l, r) = self.unwrap_pair();

            let l_exp = ExplodeCarry::Lhs(l);
            let r_exp = ExplodeCarry::Rhs(r);

            *self = Self::Number(0);
            *stable = false;

            return (Some(l_exp), Some(r_exp));
        }

        if lvl < 4 {
            if let Self::Another(l, r) = self {
                if let (Some(l_ec), Some(r_ec)) = l.explode(stable, lvl + 1) {
                    let r_ec = r.carry(&r_ec);

                    return (Some(l_ec), Some(r_ec));
                } else if let (Some(l_exp), Some(r_exp)) = r.explode(stable, lvl + 1) {
                    let l_exp = l.carry(&l_exp);

                    return (Some(l_exp), Some(r_exp));
                }
            }
        }

        (None, None)
    }

    fn carry(&mut self, carried: &ExplodeCarry) -> ExplodeCarry {
        match carried {
            ExplodeCarry::Consumed => ExplodeCarry::Consumed,
            ExplodeCarry::Lhs(c) => match self {
                Pair::Number(n) => {
                    *n += c;

                    ExplodeCarry::Consumed
                }
                Pair::Another(_, r) => r.carry(carried),
            },
            ExplodeCarry::Rhs(c) => match self {
                Pair::Number(n) => {
                    *n += c;

                    ExplodeCarry::Consumed
                }
                Pair::Another(l, _) => l.carry(carried),
            },
        }
    }

    fn split(&mut self, stable: &mut bool) {
        if !*stable {
            return;
        }

        match self {
            Pair::Number(n) => {
                if *n >= 10 {
                    let l = *n / 2;
                    let r = ((*n as f32) / 2f32).ceil() as u8;

                    *self = Pair::Another(Box::new(Pair::Number(l)), Box::new(Pair::Number(r)));
                    *stable = false;
                }
            }
            Pair::Another(l, r) => {
                l.split(stable);
                r.split(stable);
            }
        }
    }
}

#[derive(Debug)]
enum ExplodeCarry {
    Consumed,
    Lhs(u8),
    Rhs(u8),
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 4140);
    }

    #[test]
    fn part_2() {
        let r = super::part_2(INPUT);
        assert_eq!(r, 3993);
    }
}

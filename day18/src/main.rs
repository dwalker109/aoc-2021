use std::{fmt::Display, ops::Add};

use serde::{Deserialize, Serialize};

fn main() {
    println!("Hello, world!");
}

fn part_1(input: &str) -> u32 {
    let n = parse_input(input);

    for p in n {
        println!("{}", &p);
    }

    todo!();
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
        Self::Another(Box::new(self), Box::new(rhs))
    }
}

impl Pair {
    fn contains_pair(&self) -> bool {
        matches!(self, Self::Another(_, _))
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 4140);
    }
}

#[cfg(test)]
mod dev_tests {
    use super::*;

    #[test]
    fn addition() {
        let x: Pair = serde_json::from_str("[1,2]").unwrap();
        let y: Pair = serde_json::from_str("[[3,4],5]").unwrap();

        let z = x + y;

        assert_eq!(z.to_string(), "[[1,2],[[3,4],5]]");
    }

    #[test]
    fn loc() {
        let a: Pair = serde_json::from_str("[[[[[9,8],1],2],3],4]").unwrap();
        let b: Pair = serde_json::from_str("[7,[6,[5,[4,[3,2]]]]]").unwrap();
        let c: Pair = serde_json::from_str("[[6,[5,[4,[3,2]]]],1]").unwrap();
        let d: Pair = serde_json::from_str("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]").unwrap();
        dbg!(&d);
    }

    #[test]
    fn nest() {
        let a: Pair = serde_json::from_str("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]").unwrap();
        let a1 = match &a {
            Pair::Number(_) => todo!(),
            Pair::Another(l, r) => match l.as_ref() {
                Pair::Number(_) => todo!(),
                Pair::Another(l, r) => r,
            },
        };

        dbg!(&a, &a1);
    }
}

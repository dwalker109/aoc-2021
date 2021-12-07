use itertools::Itertools;
use std::cmp::{max, min};

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[logging_timer::time]
fn part_1(input: &'static str) -> u32 {
    let crabs = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .filter_map(|c| c.parse::<u32>().ok())
        .collect::<Vec<_>>();

    crabs
        .iter()
        .map(|p| {
            crabs
                .iter()
                .fold(0, |acc, curr| acc + max(p, curr) - min(p, curr))
        })
        .sorted()
        .next()
        .unwrap()
}

#[logging_timer::time]
fn part_2(input: &'static str) -> u32 {
    let crabs = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .filter_map(|c| c.parse::<u32>().ok())
        .collect::<Vec<_>>();

    (0..=crabs.iter().max().unwrap().to_owned())
        .map(|p| {
            crabs.iter().fold(0, |acc, curr| {
                let diff = max(p, *curr) - min(p, *curr);
                acc + (1..=diff).sum::<u32>()
            })
        })
        .sorted()
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 37);
    }

    #[test]
    fn part_2() {
        let r = super::part_2(INPUT);
        assert_eq!(r, 168);
    }
}

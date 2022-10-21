#![feature(is_some_and)]

use crate::burrow::{Burrow, Part};
use std::collections::BTreeMap;

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", solve(INPUT, &Part::One));
    println!("Part 2: {}", solve(INPUT, &Part::Two));
}

type Cache = BTreeMap<Vec<char>, usize>;

#[logging_timer::time]
fn solve(input: &str, part: &Part) -> usize {
    let init_burrow = Burrow::from((input, part));
    let mut init_leading_score = u16::MAX as usize;
    let mut cache = Cache::new();
    init_burrow.next_state(part, &mut init_leading_score, &mut cache);

    init_leading_score
}

mod amphipod;
mod burrow;
mod loc;

#[cfg(test)]
mod tests {
    use crate::Part;

    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::solve(INPUT, &Part::One);
        assert_eq!(r, 12521);
    }

    #[test]
    fn part_2() {
        let r = super::solve(INPUT, &Part::Two);
        assert_eq!(r, 44169);
    }
}

#![feature(is_some_and)]

use crate::burrow::{Burrow, Part};
use std::collections::BTreeMap;

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
}

type Cache = BTreeMap<Vec<char>, usize>;

#[logging_timer::time]
fn part_1(input: &str) -> usize {
    let init_burrow = Burrow::from((input, Part::One));
    let mut init_leading_score = u16::MAX as usize;
    let mut cache = Cache::new();
    init_burrow.next_state(&mut init_leading_score, &mut cache);

    init_leading_score
}

mod amphipod;
mod burrow;
mod loc;

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 12521);
    }
}

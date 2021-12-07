use std::cmp::{max, min};

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
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

    let mut r = crabs
        .iter()
        .map(|p| {
            crabs
                .iter()
                .fold(0, |acc, curr| acc + max(p, curr) - min(p, curr))
        })
        .collect::<Vec<_>>();

    r.sort_unstable();

    *r.first().unwrap()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 37);
    }
}

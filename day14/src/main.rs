use itertools::{EitherOrBoth, Itertools};
use std::collections::HashMap;

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
}

#[logging_timer::time]
fn part_1(input: &'static str) -> usize {
    let (mut template, rules) = parse_input(input);

    for _ in 0..10 {
        let tpl_c = template.clone();
        let inserts = tpl_c.iter().tuple_windows().map(|(l, r)| rules[&(*l, *r)]);

        template = template
            .into_iter()
            .zip_longest(inserts)
            .flat_map(|x| match x {
                EitherOrBoth::Both(a, b) => [a, b],
                EitherOrBoth::Left(a) => [a, '_'],
                EitherOrBoth::Right(b) => [b, '_'],
            })
            .filter(|&c| c != '_')
            .collect_vec();
    }

    let counts = template.iter().counts();

    counts.iter().max_by_key(|(_, &qty)| qty).unwrap().1
        - counts.iter().min_by_key(|(_, &qty)| qty).unwrap().1
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let (template, rest) = input.split_once("\n\n").unwrap();

    let rules = rest
        .lines()
        .map(|l| {
            let c = l.chars().collect::<Vec<_>>();

            ((c[0], c[1]), c[6])
        })
        .collect::<HashMap<_, _>>();

    (template.chars().collect(), rules)
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 1588);
    }
}

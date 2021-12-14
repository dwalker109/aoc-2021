use itertools::{EitherOrBoth, Itertools};
use std::collections::HashMap;

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
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

#[logging_timer::time]
fn part_2(input: &'static str) -> usize {
    let (template, rules) = parse_input(input);

    let mut freq = template
        .iter()
        .chain(['.'].iter()) // Hanging . to anchor final element
        .tuple_windows()
        .map(|(&l, &r)| (l, r))
        .counts();

    for _ in 0..40 {
        let mut to_add = Vec::new();
        let mut to_rem = Vec::new();

        for ((l, r), &qty) in freq.iter() {
            if let Some(ins) = rules.get(&(*l, *r)) {
                to_add.push(((*l, *ins), qty));
                to_add.push(((*ins, *r), qty));
                to_rem.push(((*l, *r), qty));
            }
        }

        for (key, qty) in to_rem {
            *freq.entry(key).or_default() -= qty;
        }

        for (key, qty) in to_add {
            *freq.entry(key).or_default() += qty;
        }
    }

    let counts = freq
        .iter()
        .filter(|(_, &n)| n > 0)
        .map(|((l, _), n)| (l, *n))
        .fold(HashMap::<char, usize>::new(), |mut acc, (l, n)| {
            *acc.entry(*l).or_default() += n;

            acc
        });

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

    #[test]
    fn part_2() {
        let r = super::part_2(INPUT);
        assert_eq!(r, 2188189693529);
    }
}

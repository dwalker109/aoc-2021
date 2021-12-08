use std::collections::HashMap;

use itertools::Itertools;

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[logging_timer::time]
fn part_1(input: &'static str) -> usize {
    let raw_signal = input.lines().map(RawSignal::from);

    raw_signal.fold(0, |acc, rs| {
        acc + rs
            .1
            .iter()
            .filter(|&output| matches!(output.len(), 2 | 4 | 3 | 7))
            .count()
    })
}

#[logging_timer::time]
fn part_2(input: &'static str) -> u32 {
    let raw_signal = input.lines().map(RawSignal::from);
    let segments_source = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let segment_permutations = segments_source.iter().permutations(7).collect_vec();

    //   3
    // 4   0
    //   5
    // 6   1
    //   2

    let segments_lookup: HashMap<u8, Vec<usize>> = [
        (0, vec![0, 1, 2, 3, 4, 6]),
        (1, vec![0, 1]),
        (2, vec![0, 2, 3, 5, 6]),
        (3, vec![0, 1, 2, 3, 5]),
        (4, vec![0, 1, 4, 5]),
        (5, vec![1, 2, 3, 4, 5]),
        (6, vec![1, 2, 3, 4, 5, 6]),
        (7, vec![0, 1, 3]),
        (8, vec![0, 1, 2, 3, 4, 5, 6]),
        (9, vec![0, 1, 2, 3, 4, 5]),
    ]
    .into_iter()
    .collect();

    raw_signal
        .into_iter()
        .map(|RawSignal(signals, outputs)| {
            let mut possibles = segment_permutations.clone();

            while possibles.len() > 1 {
                possibles.retain(|possible| {
                    'signal: for signal in &signals {
                        let check_segments = segments_lookup
                            .iter()
                            .filter(|(_, segments)| segments.len() == signal.len());

                        'seg: for (_, segments) in check_segments {
                            for &check_position in segments {
                                if !signal.contains(possible[check_position]) {
                                    continue 'seg;
                                }
                            }

                            continue 'signal;
                        }

                        return false;
                    }

                    true
                });
            }

            let found = possibles.first().unwrap();

            let decoded = outputs
                .iter()
                .map(|output_val| {
                    let lookup = output_val
                        .iter()
                        .map(|output_char| {
                            found
                                .iter()
                                .find_position(|&&found_char| output_char == found_char)
                                .unwrap()
                                .0
                        })
                        .sorted()
                        .collect_vec();

                    let result = segments_lookup.iter().find(|v| *v.1 == lookup).unwrap().0;

                    result
                })
                .join("")
                .parse::<u32>()
                .unwrap();

            decoded
        })
        .sum()
}

#[derive(Debug)]
struct RawSignal(Vec<Vec<char>>, Vec<Vec<char>>);

impl From<&'static str> for RawSignal {
    fn from(raw: &'static str) -> Self {
        let (patterns, output) = raw.split_once('|').unwrap();

        Self(
            patterns
                .trim()
                .split(' ')
                .map(|s| s.chars().collect())
                .collect(),
            output
                .trim()
                .split(' ')
                .map(|s| s.chars().collect())
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 26);
    }

    #[test]
    fn part_2() {
        let r = super::part_2(INPUT);
        assert_eq!(r, 61229)
    }
}

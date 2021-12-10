use itertools::Itertools;
use std::collections::HashMap;

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

static PAIRS: [(char, char); 4] = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

#[logging_timer::time]
fn part_1(input: &'static str) -> u32 {
    let pairs: HashMap<char, char> = PAIRS.into_iter().collect();

    let scores: HashMap<char, u32> = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
        .into_iter()
        .collect();

    let (_, corrupt) = parse_input(input);

    corrupt.iter().fold(0, |acc, curr| {
        let mut stack = Vec::with_capacity(curr.len());

        for t in curr.chars() {
            match t {
                '(' | '[' | '{' | '<' => stack.push(t),
                _ => {
                    let m = stack.pop().unwrap();

                    if t != pairs[&m] {
                        return acc + scores[&t];
                    }
                }
            }
        }

        acc
    })
}

#[logging_timer::time]
fn part_2(input: &'static str) -> u64 {
    let scores: HashMap<char, u64> = [('(', 1), ('[', 2), ('{', 3), ('<', 4)]
        .into_iter()
        .collect();

    let (incomplete, _) = parse_input(input);

    let mut results = incomplete
        .iter()
        .map(|line| {
            let mut stack = Vec::with_capacity(line.len());

            for t in line.chars() {
                match t {
                    '(' | '[' | '{' | '<' => stack.push(t),
                    _ => {
                        stack.pop();
                    }
                }
            }

            stack
                .iter()
                .rev()
                .fold(0, |acc, curr| (acc * 5) + scores[curr])
        })
        .sorted();

    results
        .nth((results.size_hint().1.unwrap()) / 2)
        .unwrap()
}

fn parse_input(input: &'static str) -> (Vec<&'static str>, Vec<&'static str>) {
    let pairs: HashMap<char, char> = PAIRS.into_iter().collect();

    input.lines().partition(|&line| {
        let mut stack = Vec::with_capacity(line.len());

        for t in line.chars() {
            match t {
                '(' | '[' | '{' | '<' => stack.push(t),
                _ => {
                    let m = stack.pop().unwrap();

                    if t != pairs[&m] {
                        return false;
                    }
                }
            }
        }

        true
    })
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 26397)
    }

    #[test]
    fn part_2() {
        let r = super::part_2(INPUT);
        assert_eq!(r, 288957)
    }
}

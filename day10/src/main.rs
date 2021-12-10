use std::collections::HashMap;

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
}

#[logging_timer::time]
fn part_1(input: &str) -> u32 {
    let pairs: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .into_iter()
        .collect();

    let scores: HashMap<char, u32> = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
        .into_iter()
        .collect();

    input.lines().fold(0, |acc, curr| {
        let tokens = curr.chars();
        let mut stack = Vec::with_capacity(curr.len());

        for t in tokens {
            match t {
                '(' | '[' | '{' | '<' => stack.push(t),
                _ => {
                    let m = stack.pop().unwrap();
                    let close = pairs[&m];
                    let score = scores[&t];

                    if t != close {
                        return acc + score;
                    }
                }
            }
        }

        acc
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
}

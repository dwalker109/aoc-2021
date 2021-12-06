use std::{
    cmp::{max, min},
    collections::HashMap,
};

static INPUT: &str = include_str!("../input");

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

fn part_1(input: &'static str) -> u32 {
    let coordinates = parse_input(input);
    let mut vents = HashMap::new();

    for ((x1, y1), (x2, y2)) in coordinates
        .into_iter()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
    {
        for x in min(x1, x2)..=max(x1, x2) {
            for y in min(y1, y2)..=max(y1, y2) {
                *vents.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    vents.values().filter(|&v| *v > 1).count() as u32
}
fn part_2(input: &'static str) -> u32 {
    let coordinates = parse_input(input);
    let mut vents = HashMap::new();

    for ((x1, y1), (x2, y2)) in coordinates {
        match x1 == x2 || y1 == y2 {
            true => {
                for x in min(x1, x2)..=max(x1, x2) {
                    for y in min(y1, y2)..=max(y1, y2) {
                        *vents.entry((x, y)).or_insert(0) += 1;
                    }
                }
            }
            false => {
                let x_range = min(x1, x2)..=max(x1, x2);
                let xs = match x2 > x1 {
                    true => x_range.collect::<Vec<_>>(),
                    false => x_range.rev().collect::<Vec<_>>(),
                };

                let y_range = min(y1, y2)..=max(y1, y2);
                let ys = match y2 > y1 {
                    true => y_range.collect::<Vec<_>>(),
                    false => y_range.rev().collect::<Vec<_>>(),
                };

                for (x, y) in xs.iter().zip(ys.iter()) {
                    *vents.entry((*x, *y)).or_insert(0) += 1;
                }
            }
        }
    }

    vents.values().filter(|&v| *v > 1).count() as u32
}

fn parse_input(input: &'static str) -> Vec<((u32, u32), (u32, u32))> {
    let exp = regex::Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    exp.captures_iter(input)
        .map(|caps| {
            let c = caps
                .iter()
                .skip(1)
                .map(|c| c.unwrap().as_str().parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            ((c[0], c[1]), (c[2], c[3]))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 5);
    }

    #[test]
    fn part_2() {
        let r = super::part_2(INPUT);
        assert_eq!(r, 12);
    }
}

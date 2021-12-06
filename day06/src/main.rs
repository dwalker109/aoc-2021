use std::mem::replace;

static INPUT: &str = include_str!("../input");

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

fn part_1(input: &'static str) -> usize {
    solve(input, 80)
}

fn part_2(input: &'static str) -> usize {
    solve(input, 256)
}

fn solve(input: &'static str, days: u32) -> usize {
    let mut fish = vec![0; 9];

    for f in input.lines().next().unwrap().split(',') {
        let f = f.parse::<usize>().unwrap();
        fish[f] += 1;
    }

    for _ in 1..=days {
        let from_0 = replace(&mut fish[0], 0);

        for i in 1..=8 {
            let f = replace(&mut fish[i], 0);
            fish[i - 1] = f;
        }

        fish[6] += from_0;
        fish[8] = from_0;
    }

    fish.iter().sum()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 5934);
    }

    #[test]
    fn part_2() {
        let r = super::part_2(INPUT);
        assert_eq!(r, 26984457539);
    }
}

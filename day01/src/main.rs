use itertools::Itertools;

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[logging_timer::time]
fn part_1(input: &'static str) -> u32 {
    input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .tuple_windows()
        .fold(0, |acc, (l, r)| acc + (r > l) as u32)
}

#[logging_timer::time]
fn part_2(input: &'static str) -> u32 {
    input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .fold(0, |acc, (l, r)| acc + (r > l) as u32)
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 7);
    }

    #[test]
    fn part_2() {
        let r = super::part_2(INPUT);
        assert_eq!(r, 5);
    }
}

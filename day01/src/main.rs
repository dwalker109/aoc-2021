use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input")));
    println!("Part 2: {}", part_2(include_str!("../input")));
}

fn part_1(input: &'static str) -> u32 {
    input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .tuple_windows()
        .fold(0, |acc, (l, r)| acc + (r > l) as u32)
}

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
    #[test]
    fn part_1() {
        let r = super::part_1(include_str!("../input"));
        assert_eq!(r, 1390);
    }

    #[test]
    fn part_2() {
        let r = super::part_2(include_str!("../input"));
        assert_eq!(r, 1457);
    }
}

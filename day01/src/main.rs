use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input")));
}

fn part_1(input: &'static str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .tuple_windows()
        .fold(0_usize, |acc, (l, r)| acc + (r > l) as usize)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let r = super::part_1(include_str!("../input_test"));
        assert_eq!(r, 7);
    }
}

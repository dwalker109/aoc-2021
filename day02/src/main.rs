use aoc_support::input::DAY_02 as INPUT;

fn main() {
    println!("Part 1: {}", part_1(&INPUT));
}

fn part_1(input: &'static str) -> u32 {
    let (hp, d) = input
        .lines()
        .map(|l| {
            let mut x = l.split_ascii_whitespace();
            (x.next().unwrap(), x.next().unwrap().parse::<u32>().unwrap())
        })
        .fold((0, 0), |acc, (instr, n)| match instr {
            "forward" => (acc.0 + n, acc.1),
            "down" => (acc.0, acc.1 + n),
            "up" => (acc.0, acc.1 - n),
            _ => panic!(),
        });

    hp * d
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 150);
    }
}

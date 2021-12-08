static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
}

#[logging_timer::time]
fn part_1(input: &'static str) -> usize {
    let raw_signal = input.lines().map(RawSignal::from).collect::<Vec<_>>();

    raw_signal.iter().fold(0, |acc, rs| {
        acc + rs
            .1
            .iter()
            .filter(|&&ov| matches!(ov.len(), 2 | 4 | 3 | 7))
            .count()
    })
}

#[derive(Debug)]
struct RawSignal(Vec<&'static str>, Vec<&'static str>);

impl From<&'static str> for RawSignal {
    fn from(raw: &'static str) -> Self {
        let (patterns, output) = raw.split_once('|').unwrap();

        Self(
            patterns.trim().split(' ').collect(),
            output.trim().split(' ').collect(),
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
}

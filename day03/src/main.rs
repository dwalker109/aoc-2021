use itertools::Itertools;

static INPUT: &str = include_str!("../input");

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

fn part_1(input: &'static str) -> u32 {
    let line_len = input.split_once("\n").unwrap().0.len();

    let counts = (0..line_len)
        .map(|i| {
            input
                .replace("\n", "")
                .chars()
                .skip(i)
                .step_by(line_len)
                .counts()
                .into_iter()
                .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let gamma = u32::from_str_radix(&counts.iter().map(|x| x[0].0).join(""), 2).unwrap();
    let epsilon = u32::from_str_radix(&counts.iter().map(|x| x[1].0).join(""), 2).unwrap();

    gamma * epsilon
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 198);
    }
}

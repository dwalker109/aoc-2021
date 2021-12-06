use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[logging_timer::time]
fn part_1(input: &'static str) -> u32 {
    let bitness = input.lines().next().unwrap().len();
    let bits = parse_input(input);
    let bit_pop = (0..bitness)
        .map(|pos| get_bit_popularity(&bits, pos))
        .collect::<Vec<_>>();

    let gamma = u32::from_str_radix(&bit_pop.iter().map(|x| x[1].0).join(""), 2).unwrap();
    let epsilon = u32::from_str_radix(&bit_pop.iter().map(|x| x[0].0).join(""), 2).unwrap();

    gamma * epsilon
}

#[logging_timer::time]
fn part_2(input: &str) -> u32 {
    let bitness = input.lines().next().unwrap().len();
    let bits = parse_input(input);

    let oxy = find_via_bit_criteria(bitness, &bits, 1).iter().join("");
    let co2 = find_via_bit_criteria(bitness, &bits, 0).iter().join("");

    let oxy = u32::from_str_radix(&oxy, 2).unwrap();
    let co2 = u32::from_str_radix(&co2, 2).unwrap();

    oxy * co2
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .filter(|c| !c.is_empty())
        .collect()
}

fn get_bit_popularity(input: &[Vec<char>], pos: usize) -> Vec<(char, usize)> {
    let mut counts = input.iter().map(|i| i[pos]).counts();
    counts.entry('0').or_insert(0);
    counts.entry('1').or_insert(0);

    counts
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
        .collect_vec()
}

fn find_via_bit_criteria(bitness: usize, bits: &[Vec<char>], winner: usize) -> Vec<char> {
    let default = char::from_digit(winner as u32, 2).unwrap();

    (0..bitness)
        .fold_while(bits.to_owned(), |rem_bits, pos| {
            let bit_pop = get_bit_popularity(&rem_bits, pos);
            let to_keep = match bit_pop[0].1 == bit_pop[1].1 {
                true => default,
                false => bit_pop[winner].0,
            };
            let rem = rem_bits
                .into_iter()
                .filter(|curr| curr[pos] == to_keep)
                .collect_vec();

            if rem.len() > 1 {
                Continue(rem)
            } else {
                Done(rem)
            }
        })
        .into_inner()
        .into_iter()
        .flatten()
        .collect_vec()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 198);
    }

    #[test]
    fn part_2() {
        let r = super::part_2(INPUT);
        assert_eq!(r, 230);
    }
}

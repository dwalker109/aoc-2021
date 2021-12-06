static INPUT: &str = include_str!("../input");

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

fn part_1(input: &'static str) -> usize {
    let mut fish = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|f| f.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..80 {
        let mut new_fish = vec![];
        for f in fish.iter_mut() {
            match f {
                0 => {
                    *f = 6;
                    new_fish.push(8);
                }
                _ => *f -= 1,
            }
        }
        fish.append(&mut new_fish);
    }

    fish.len()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 5934);
    }
}

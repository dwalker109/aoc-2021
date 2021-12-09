use std::collections::HashMap;

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
}

#[logging_timer::time]
fn part_1(input: &'static str) -> u32 {
    let coordinates = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c.to_digit(10).unwrap()))
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<_, _>>();

    coordinates
        .iter()
        .filter_map(|((x, y), v)| {
            let up = coordinates.get(&(*x, *y - 1)).unwrap_or(&u32::MAX);
            let down = coordinates.get(&(*x, *y + 1)).unwrap_or(&u32::MAX);
            let left = coordinates.get(&(*x - 1, *y)).unwrap_or(&u32::MAX);
            let right = coordinates.get(&(*x + 1, *y)).unwrap_or(&u32::MAX);

            if v < up && v < down && v < left && v < right {
                Some(1 + v)
            } else {
                None
            }
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 15);
    }
}

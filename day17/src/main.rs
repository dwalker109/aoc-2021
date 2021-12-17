use std::ops::RangeInclusive;

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[logging_timer::time]
fn part_1(input: &str) -> i32 {
    let (x_range, y_range) = parse_input(input);

    let mut results = Vec::new();

    for vx in 0..(*x_range.end()) {
        for vy in 0..(y_range.start().abs()) {
            results.push(simulate(&x_range, &y_range, vx, vy));
        }
    }

    results.iter().filter_map(|v| *v).max().unwrap()
}

#[logging_timer::time]
fn part_2(input: &str) -> usize {
    let (x_range, y_range) = parse_input(input);

    let mut count = 0;

    for vx in 1..=(*x_range.end()) {
        for vy in (y_range.start() * 3)..(y_range.start().abs() * 3) {
            if simulate(&x_range, &y_range, vx, vy).is_some() {
                count += 1;
            }
        }
    }

    count
}

fn simulate(
    x_range: &RangeInclusive<i32>,
    y_range: &RangeInclusive<i32>,
    mut vx: i32,
    mut vy: i32,
) -> Option<i32> {
    let mut x = 0i32;
    let mut y = 0i32;
    let mut max_y = 0;

    loop {
        x += vx;
        y += vy;
        vx += match vx.cmp(&0) {
            std::cmp::Ordering::Greater => -1,
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
        };
        vy -= 1;

        if x_range.contains(&x) && y_range.contains(&y) {
            return Some(max_y);
        }

        if (x < *x_range.start() && vx == 0) || x > *x_range.end() || y < *y_range.start() {
            return None;
        }

        if y > max_y {
            max_y = y;
        }
    }
}

fn parse_input(input: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let re = regex::Regex::new(r"x=(-?\d+)\.\.(-?\d+).*y=(-?\d+)\.\.(-?\d+)").unwrap();
    let caps = re.captures(input).unwrap();
    let [from_x, to_x, from_y, to_y] = <[i32; 4]>::try_from(
        caps.iter()
            .skip(1)
            .filter_map(|n| n.unwrap().as_str().parse::<i32>().ok())
            .collect::<Vec<_>>(),
    )
    .unwrap();

    (from_x..=to_x, from_y..=to_y)
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 45);
    }

    #[test]
    fn part_2() {
        let r = super::part_2(INPUT);
        assert_eq!(r, 112);
    }
}

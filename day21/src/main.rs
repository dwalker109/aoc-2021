static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
}

#[logging_timer::time]
fn part_1(input: &str) -> usize {
    let (mut p1_pos, mut p2_pos) = parse_input(input);

    let mut dd = (1..=100).cycle();
    let mut dd_rolls = 0u32;
    const ROLL_QTY: u8 = 3;

    let mut p1_score = 0usize;
    let mut p2_score = 0usize;

    loop {
        let p1_dd = dd.by_ref().take(ROLL_QTY as usize).sum::<usize>();
        dd_rolls += ROLL_QTY as u32;
        p1_pos = (((p1_pos as usize + p1_dd - 1) % 10) + 1) as u8;
        p1_score += p1_pos as usize;

        if p1_score >= 1000 {
            break;
        }

        let p2_dd = dd.by_ref().take(ROLL_QTY as usize).sum::<usize>();
        dd_rolls += ROLL_QTY as u32;
        p2_pos = (((p2_pos as usize + p2_dd - 1) % 10) + 1) as u8;
        p2_score += p2_pos as usize;

        if p2_score >= 1000 {
            break;
        }
    }

    std::cmp::min(p1_score, p2_score) * dd_rolls as usize
}

fn parse_input(input: &str) -> (u8, u8) {
    let dice = input
        .lines()
        .map(|l| l.chars().last().unwrap().to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();

    (dice[0], dice[1])
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 739785);
    }
}

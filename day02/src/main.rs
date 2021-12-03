static INPUT: &str = include_str!("../input");

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

fn part_1(input: &'static str) -> u32 {
    let (h, d) = input
        .lines()
        .map(Instruction::from)
        .fold((0, 0), |(h, d), instruction| match instruction {
            Instruction::Forward(n) => (h + n, d),
            Instruction::Down(n) => (h, d + n),
            Instruction::Up(n) => (h, d - n),
        });

    h * d
}

fn part_2(input: &'static str) -> u32 {
    let (h, d, _) =
        input
            .lines()
            .map(Instruction::from)
            .fold((0, 0, 0), |(h, d, a), instruction| match instruction {
                Instruction::Forward(n) => (h + n, d + a * n, a),
                Instruction::Down(n) => (h, d, a + n),
                Instruction::Up(n) => (h, d, a - n),
            });

    h * d
}

enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        let [text, num] =
            <[&str; 2]>::try_from(line.split_ascii_whitespace().collect::<Vec<_>>()).unwrap();

        match text {
            "forward" => Self::Forward(num.parse::<u32>().unwrap()),
            "down" => Self::Down(num.parse::<u32>().unwrap()),
            "up" => Self::Up(num.parse::<u32>().unwrap()),
            _ => panic!(),
        }
    }
}
#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 150);
    }
    #[test]
    fn part_2() {
        let r = super::part_2(INPUT);
        assert_eq!(r, 900);
    }
}

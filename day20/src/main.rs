use std::{collections::HashMap, fmt::Display};

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[logging_timer::time]
fn part_1(input: &str) -> usize {
    solve(input, 2)
}

#[logging_timer::time]
fn part_2(input: &str) -> usize {
    solve(input, 50)
}

fn solve(input: &str, qty: usize) -> usize {
    let (alg, mut img) = parse_input(input);

    for n in 0..(qty as isize) {
        img = img.enhance(&alg, n);
    }

    img.num_lit_pixel()
}

fn parse_input(input: &str) -> (Alg, Img) {
    let (alg, input_img) = input.split_once("\n\n").unwrap();

    let alg = Alg(alg.chars().map(Pxl::from).collect());

    let input_img = Img(input_img
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, p)| ((x as isize, y as isize), Pxl::from(p)))
        })
        .collect());

    (alg, input_img)
}

#[derive(Clone, Copy)]
struct Pxl(bool);

impl From<Pxl> for char {
    fn from(val: Pxl) -> Self {
        match val.0 {
            true => '#',
            false => '.',
        }
    }
}

impl From<char> for Pxl {
    fn from(val: char) -> Self {
        match val {
            '.' => Pxl(false),
            '#' => Pxl(true),
            _ => unreachable!(),
        }
    }
}

struct Alg(Vec<Pxl>);

struct Img(HashMap<(isize, isize), Pxl>);

impl Display for Img {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.0.iter().map(|(xy, _)| xy.0).min().unwrap();
        let max_x = self.0.iter().map(|(xy, _)| xy.0).max().unwrap();
        let min_y = self.0.iter().map(|(xy, _)| xy.1).min().unwrap();
        let max_y = self.0.iter().map(|(xy, _)| xy.1).max().unwrap();

        let mut out = String::with_capacity(self.0.len());

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                out.push(char::from(*self.0.get(&(x, y)).unwrap()));
            }
            out.push('\n')
        }

        write!(f, "{}", out)
    }
}

impl Img {
    fn pixel_to_int(&self, centre: &(isize, isize), default: &Pxl) -> u32 {
        let (cx, cy) = centre;

        (cy - 1..=cy + 1)
            .flat_map(|y| {
                (cx - 1..=cx + 1).map(move |x| *self.0.get(&(x, y)).unwrap_or(default))
            }).fold(0, |res, Pxl(bit)| (res << 1) ^ (bit as u32))
    }

    fn enhance(&self, alg: &Alg, iter: isize) -> Img {
        let min_x = self.0.iter().map(|(xy, _)| xy.0).min().unwrap();
        let max_x = self.0.iter().map(|(xy, _)| xy.0).max().unwrap();
        let min_y = self.0.iter().map(|(xy, _)| xy.1).min().unwrap();
        let max_y = self.0.iter().map(|(xy, _)| xy.1).max().unwrap();

        let default = match iter % 2 == 0 {
            true => alg.0.last().unwrap(),
            false => alg.0.first().unwrap(),
        };

        let mut out = Img(HashMap::with_capacity((max_x * max_y + 2 * max_x) as usize));

        for y in min_y - 1..=max_y + 1 {
            for x in min_x - 1..=max_x + 1 {
                let algo_idx = &self.pixel_to_int(&(x, y), default);
                out.0.insert((x, y), alg.0[*algo_idx as usize]);
            }
        }

        out
    }

    fn num_lit_pixel(&self) -> usize {
        self.0.iter().map(|(_, p)| p.0 as usize).sum()
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 28);
    }

    #[test]
    fn part_2() {
        let r = super::part_2(INPUT);
        assert_eq!(r, 3798);
    }
}

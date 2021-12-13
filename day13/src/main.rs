use std::{collections::{HashSet, VecDeque}, fmt::Display};

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[logging_timer::time]
fn part_1(input: &'static str) -> u32 {
    let mut paper = Paper::from(input);
    paper.origami(Some(1));

    paper.dots.len() as u32
}

#[logging_timer::time]
fn part_2(input: &'static str) -> String {
    let mut paper = Paper::from(input);
    paper.origami(None);

    paper.to_string()
}

struct Paper {
    dots: HashSet<Dot>,
    folds: VecDeque<Fold>,
}

impl From<&str> for Paper {
    fn from(raw: &str) -> Self {
        let (input_dots, input_folds) = raw.split_once("\n\n").unwrap();

        Paper {
            dots: input_dots.lines().map(Dot::from).collect::<HashSet<Dot>>(),
            folds: input_folds.lines().map(Fold::from).collect::<VecDeque<_>>(),
        }
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.dots.iter().map(|Dot(x, _)| *x).max().unwrap();
        let max_y = self.dots.iter().map(|Dot(_, y)| *y).max().unwrap();

        let mut output = '\n'.to_string();

        for y in 0..=max_y {
            for x in 0..=max_x {
                output.push_str(&match self.dots.contains(&Dot(x, y)) {
                    true => '█'.to_string(),
                    false => '░'.to_string(),
                });
            }
            output.push_str(&'\n'.to_string())
        }

        write!(f, "{}", output)
    }
}

impl Paper {
    fn origami(&mut self, n: Option<usize>) {
        let mut n = match n {
            Some(n) => std::cmp::min(n, self.folds.len()),
            None => self.folds.len(),
        };

        while n > 0 {
            let fold = self.folds.pop_front().unwrap();
            let mut new = HashSet::with_capacity(self.dots.len());

            for Dot(x, y) in self.dots.iter() {
                let (tx, ty) = match fold {
                    Fold::Left(n) => match x > &n {
                        true => (n - (x - n), *y),
                        false => (*x, *y),
                    },
                    Fold::Up(n) => match y > &n {
                        true => (*x, n - (y - n)),
                        false => (*x, *y),
                    },
                };

                new.insert(Dot(tx, ty));
            }

            self.dots = new;
            n -= 1
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Dot(u32, u32);

impl From<&str> for Dot {
    fn from(raw: &str) -> Self {
        let (x, y) = raw.split_once(',').unwrap();

        Self(x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
    }
}

enum Fold {
    Left(u32),
    Up(u32),
}

impl From<&str> for Fold {
    fn from(raw: &str) -> Self {
        let (axis, n) = raw.split_once('=').unwrap();
        match axis.contains('x') {
            true => Self::Left(n.parse().unwrap()),
            _ => Self::Up(n.parse().unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 17);
    }
    
    #[test]
    fn part_2() {
        let r = super::part_2(INPUT);
        assert_eq!(format!("{}", r), "\n█████\n█░░░█\n█░░░█\n█░░░█\n█████\n");
    }
}

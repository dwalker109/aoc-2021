use std::{cell::RefCell, collections::HashMap, fmt::Display};

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
}

#[logging_timer::time]
fn part_1(input: &'static str) -> u32 {
    let mut cavern = Cavern(
        input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(move |(x, c)| {
                        (
                            (x as isize, y as isize),
                            RefCell::new(c.to_digit(10).unwrap()),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashMap<_, _>>(),
    );

    let mut flashes = 0;

    for _ in 1..=100 {
        for (_, e) in cavern.0.iter_mut() {
            *e.borrow_mut() += 1;
        }

        let mut needs_scan = true;
        while needs_scan {
            needs_scan = false;

            for ((x, y), e) in cavern
                .0
                .iter()
                .filter(|(_, e)| *(*e).borrow() != 0 && *(*e).borrow() > 9)
            {
                for x in x - 1..=x + 1 {
                    for y in y - 1..=y + 1 {
                        if let Some(e) = cavern.0.get(&(x, y)) {
                            if *e.borrow_mut() != 0 {
                                *e.borrow_mut() += 1;
                            }
                        }
                    }
                }

                flashes += 1;
                *e.borrow_mut() = 0;
                needs_scan = true;
            }
        }
    }

    flashes
}

#[derive(Debug)]
struct Cavern(HashMap<(isize, isize), RefCell<u32>>);

impl Display for Cavern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for y in 0..10isize {
            for x in 0..10isize {
                let c = *self.0.get(&(x, y)).unwrap().borrow();
                out.push_str(&c.to_string());
            }
            out.push('\n');
        }

        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 1656)
    }
}

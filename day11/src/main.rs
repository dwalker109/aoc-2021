use std::{cell::RefCell, collections::HashMap};

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[logging_timer::time]
fn part_1(input: &'static str) -> u32 {
    let mut cavern = Cavern::from(input);

    let mut flashes = 0;

    for _ in 1..=100 {
        cavern.start_step();

        while cavern.rescan_needed() {
            for octopus in cavern.will_flash() {
                cavern.flash_neighbours(octopus);
                flashes += 1;
            }
        }
    }

    flashes
}

#[logging_timer::time]
fn part_2(input: &'static str) -> u32 {
    let mut cavern = Cavern::from(input);

    for step in 1.. {
        cavern.start_step();

        while cavern.rescan_needed() {
            for octopus in cavern.will_flash() {
                cavern.flash_neighbours(octopus);
            }

            if cavern.was_simultaneous_flash() {
                return step;
            }
        }
    }

    unreachable!();
}

struct Cavern {
    octopi: HashMap<(isize, isize), RefCell<u32>>,
    settled: RefCell<bool>,
}

impl From<&'static str> for Cavern {
    fn from(input: &'static str) -> Cavern {
        Self {
            octopi: input
                .lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars().enumerate().map(move |(x, c)| {
                        (
                            (x as isize, y as isize),
                            RefCell::new(c.to_digit(10).unwrap()),
                        )
                    })
                })
                .collect::<HashMap<_, _>>(),

            settled: RefCell::new(true),
        }
    }
}

impl Cavern {
    fn start_step(&mut self) {
        for (_, e) in &self.octopi {
            *e.borrow_mut() += 1;
        }

        *self.settled.borrow_mut() = false;
    }

    fn rescan_needed(&self) -> bool {
        *self.settled.borrow() == false
    }

    fn will_flash(&self) -> impl Iterator<Item = (&(isize, isize), &RefCell<u32>)> {
        let iter = self
            .octopi
            .iter()
            .filter(|(_, energy)| *(energy).borrow() != 0 && *(energy).borrow() > 9);

        if iter.clone().count() == 0 {
            *self.settled.borrow_mut() = true;
        } else {
            *self.settled.borrow_mut() = false;
        }

        iter
    }

    fn flash_neighbours(&self, ((s_x, s_y), s_energy): (&(isize, isize), &RefCell<u32>)) {
        for x in s_x - 1..=s_x + 1 {
            for y in s_y - 1..=s_y + 1 {
                if let Some(energy) = self.octopi.get(&(x, y)) {
                    if *energy.borrow_mut() != 0 {
                        *energy.borrow_mut() += 1;
                    }
                }
            }
        }

        *s_energy.borrow_mut() = 0;
    }

    fn was_simultaneous_flash(&self) -> bool {
        if self.octopi.iter().all(|(_, e)| *(*e).borrow() == 0) {
            return true;
        }

        false
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

    #[test]
    fn part_2() {
        let r = super::part_2(INPUT);
        assert_eq!(r, 195)
    }
}

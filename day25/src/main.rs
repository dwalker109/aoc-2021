use rustc_hash::{FxHashMap, FxHashSet};

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
}

#[logging_timer::time]
fn part_1(input: &str) -> usize {
    let mut herds = Herds::from(input);

    for i in 1usize.. {
        herds.sim();

        if herds.is_settled {
            return i;
        }
    }

    unreachable!();
}

#[derive(Clone, Copy)]
enum SeaCucumber {
    East,
    South,
}

impl From<char> for SeaCucumber {
    fn from(c: char) -> Self {
        match c {
            '>' => Self::East,
            'v' => Self::South,
            _ => unreachable!(),
        }
    }
}

impl SeaCucumber {
    fn try_move(&self, curr_xy: &(usize, usize), herds: &Herds) -> (usize, usize) {
        let next_xy = match self {
            SeaCucumber::East => ((curr_xy.0 + 1) % herds.max_x, curr_xy.1),
            SeaCucumber::South => (curr_xy.0, (curr_xy.1 + 1) % herds.max_y),
        };

        match herds.members_east.contains(&next_xy) || herds.members_south.contains(&next_xy) {
            true => *curr_xy,
            false => next_xy,
        }
    }
}

struct Herds {
    max_x: usize,
    max_y: usize,
    members_east: FxHashSet<(usize, usize)>,
    members_south: FxHashSet<(usize, usize)>,
    is_settled: bool,
}

impl From<&str> for Herds {
    fn from(raw: &str) -> Self {
        let max_x = raw.lines().next().unwrap().len();
        let max_y = raw.lines().count();

        let members = raw
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .filter(|(_, c)| ['>', 'v'].contains(c))
                    .map(move |(x, c)| ((x, y), SeaCucumber::from(c)))
            })
            .collect::<FxHashMap<_, _>>();

        Self {
            max_x,
            max_y,
            members_east: members
                .iter()
                .filter(|(_, sc)| matches!(sc, SeaCucumber::East))
                .map(|(xy, _)| (*xy))
                .collect(),
            members_south: members
                .iter()
                .filter(|(_, sc)| matches!(sc, SeaCucumber::South))
                .map(|(xy, _)| (*xy))
                .collect(),
            is_settled: false,
        }
    }
}

impl Herds {
    fn sim(&mut self) {
        self.is_settled = true;

        let mut next_east = self.members_east.clone();
        for xy in self.members_east.iter() {
            let next_xy = SeaCucumber::East.try_move(xy, self);
            if xy != &next_xy {
                next_east.remove(xy);
                next_east.insert(next_xy);
                self.is_settled = false;
            }
        }
        self.members_east = next_east;

        let mut next_south = self.members_south.clone();
        for xy in self.members_south.iter() {
            let next_xy = SeaCucumber::South.try_move(xy, self);
            if xy != &next_xy {
                next_south.remove(xy);
                next_south.insert(next_xy);
                self.is_settled = false;
            }
        }
        self.members_south = next_south;
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 58);
    }
}

use itertools::Itertools;
use std::collections::HashMap;

static INPUT: &str = include_str!("../input");

const START_ID: usize = 0;
const END_ID: usize = 20; // Input size is known to fit within this

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[logging_timer::time]
fn part_1(input: &'static str) -> usize {
    let all_caves = parse_input_caves(input);
    let cave_conns = parse_input_cave_conns(input, &all_caves);
    let cave_map = [false; END_ID + 1];

    cave_conns.explore(0, 0, cave_map).len()
}

#[logging_timer::time]
fn part_2(input: &'static str) -> usize {
    let all_caves = parse_input_caves(input);
    let cave_conns = parse_input_cave_conns(input, &all_caves);
    let cave_map = [false; END_ID + 1];

    cave_conns.explore(0, 1, cave_map).len()
}

fn parse_input_caves(input: &'static str) -> AllCaves {
    let mut caves = input
        .lines()
        .flat_map(|l| l.split('-').map(|c| (c, Cave::from(c))))
        .unique_by(|c| c.0)
        .collect::<HashMap<_, _>>();

    let mut count_id = 0;

    for c in caves.iter_mut() {
        c.1.id = match c.1.name {
            "start" => START_ID,
            "end" => END_ID,
            _ => {
                count_id += 1;
                count_id
            }
        };
    }

    AllCaves(caves)
}

fn parse_input_cave_conns<'a>(input: &'static str, all_caves: &'a AllCaves) -> CaveConns<'a> {
    let mut cave_conns = CaveConns(
        all_caves
            .0
            .iter()
            .map(|(_, cave)| (cave.id, Vec::new()))
            .collect(),
    );

    for line in input.lines() {
        let (id_a, id_b) = line.split_once('-').unwrap();

        let cave_a = all_caves.0.get(id_a).unwrap();
        let cave_b = all_caves.0.get(id_b).unwrap();

        cave_conns.0.get_mut(&cave_a.id).unwrap().push(cave_b);
        cave_conns.0.get_mut(&cave_b.id).unwrap().push(cave_a);
    }

    cave_conns
}

struct Cave {
    id: usize,
    name: &'static str,
    big: bool,
}

impl<'a> From<&'static str> for Cave {
    fn from(raw: &'static str) -> Self {
        Cave {
            id: 0,
            name: raw,
            big: raw.to_ascii_uppercase() == raw,
        }
    }
}

type CaveMap = [bool; END_ID + 1];

struct AllCaves(HashMap<&'static str, Cave>);

struct CaveConns<'a>(HashMap<usize, Vec<&'a Cave>>);

impl CaveConns<'_> {
    fn explore(&self, root: usize, upgrades: u8, mut trail: CaveMap) -> Vec<CaveMap> {
        trail[root] = true;

        if root == END_ID {
            return vec![trail];
        }

        self.0[&root]
            .iter()
            .filter(|&conn| conn.id != START_ID)
            .filter_map(|conn| {
                if conn.big || !trail[conn.id] {
                    return Some(self.explore(conn.id, upgrades, trail));
                }

                if upgrades > 0 {
                    return Some(self.explore(conn.id, upgrades - 1, trail));
                }

                None
            })
            .flatten()
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    static INPUT_1: &str = include_str!("../input_test1");
    static INPUT_2: &str = include_str!("../input_test2");
    static INPUT_3: &str = include_str!("../input_test3");

    #[test]
    fn part_1() {
        let r1 = super::part_1(INPUT_1);
        let r2 = super::part_1(INPUT_2);
        let r3 = super::part_1(INPUT_3);
        assert_eq!(r1, 10);
        assert_eq!(r2, 19);
        assert_eq!(r3, 226);
    }

    #[test]
    fn part_2() {
        let r1 = super::part_2(INPUT_1);
        let r2 = super::part_2(INPUT_2);
        let r3 = super::part_2(INPUT_3);
        assert_eq!(r1, 36);
        assert_eq!(r2, 103);
        assert_eq!(r3, 3509);
    }
}

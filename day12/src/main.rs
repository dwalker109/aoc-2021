use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::rc::Rc;

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[logging_timer::time]
fn part_1(input: &'static str) -> usize {
    let all_caves = parse_input_caves(input);
    let cave_conns = parse_input_cave_conns(input, &all_caves);

    cave_conns
        .explore("start", 0, &mut CaveMap(FxHashSet::default()))
        .len()
}

#[logging_timer::time]
fn part_2(input: &'static str) -> usize {
    let all_caves = parse_input_caves(input);
    let cave_conns = parse_input_cave_conns(input, &all_caves);

    cave_conns
        .explore("start", 1, &mut CaveMap(FxHashSet::default()))
        .len()
}

fn parse_input_caves(input: &'static str) -> AllCaves {
    AllCaves(
        input
            .lines()
            .flat_map(|l| l.split('-').map(|c| (c, Cave::from(c))))
            .unique_by(|c| c.0)
            .collect::<FxHashMap<_, _>>(),
    )
}

fn parse_input_cave_conns<'a>(input: &'static str, all_caves: &'a AllCaves) -> CaveConns<'a> {
    let mut cave_conns = CaveConns(all_caves.0.iter().map(|(&c, _)| (c, Vec::new())).collect());

    for line in input.lines() {
        let (id_a, id_b) = line.split_once('-').unwrap();

        let cave_a = all_caves.0.get(id_a).unwrap();
        let cave_b = all_caves.0.get(id_b).unwrap();

        cave_conns.0.get_mut(id_a).unwrap().push(cave_b);
        cave_conns.0.get_mut(id_b).unwrap().push(cave_a);
    }

    cave_conns
}

struct Cave {
    name: &'static str,
    big: bool,
}

impl<'a> From<&'static str> for Cave {
    fn from(raw: &'static str) -> Self {
        Cave {
            name: raw,
            big: raw.to_ascii_uppercase() == raw,
        }
    }
}

#[derive(Clone)]
struct CaveMap(FxHashSet<&'static str>);

struct AllCaves(FxHashMap<&'static str, Cave>);

struct CaveConns<'a>(FxHashMap<&'static str, Vec<&'a Cave>>);

impl CaveConns<'_> {
    fn explore(&self, root: &'static str, upgrades: u8, trail: &mut CaveMap) -> Vec<CaveMap> {
        trail.0.insert(root);

        if root == "end" {
            return vec![trail.to_owned()];
        }

        self.0[root]
            .iter()
            .filter(|&conn| conn.name != "start")
            .filter_map(|conn| {
                if conn.big || !trail.0.contains(&conn.name) {
                    return Some(self.explore(conn.name, upgrades, &mut trail.clone()));
                }

                if upgrades > 0 {
                    return Some(self.explore(conn.name, upgrades - 1, &mut trail.clone()));
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

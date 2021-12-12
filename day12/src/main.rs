use itertools::Itertools;
use std::{collections::HashMap, rc::Rc};

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
}

#[logging_timer::time]
fn part_1(input: &'static str) -> usize {
    let caves = input
        .lines()
        .flat_map(|l| l.split('-').map(|c| (c, Rc::new(Cave::from(c)))))
        .unique_by(|c| c.0)
        .collect::<HashMap<&'static str, Rc<Cave>>>();

    let mut cave_conns = CaveConns(caves.iter().map(|(&c, _)| (c, Vec::new())).collect());

    for line in input.lines() {
        let (id_a, id_b) = line.split_once('-').unwrap();

        let cave_a = caves.get(id_a).unwrap();
        let cave_b = caves.get(id_b).unwrap();

        cave_conns.0.get_mut(id_a).unwrap().push(Rc::clone(cave_b));
        cave_conns.0.get_mut(id_b).unwrap().push(Rc::clone(cave_a));
    }

    cave_conns.explore("start", &mut CaveMap::new()).len()
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug)]
struct CaveConns(HashMap<&'static str, Vec<Rc<Cave>>>);

impl CaveConns {
    fn explore(&self, from_cave: &'static str, breadcrumbs: &mut CaveMap) -> Vec<CaveMap> {
        breadcrumbs.0.push(from_cave);

        if from_cave == "end" {
            return vec![breadcrumbs.to_owned()];
        }

        self.0[from_cave]
            .iter()
            .filter(|&prospect| prospect.name != "start")
            .filter(|&prospect| {
                prospect.big
                    || !breadcrumbs
                        .0
                        .iter()
                        .any(|&visited| visited == prospect.name)
            })
            .flat_map(|prospect| self.explore(prospect.name, &mut breadcrumbs.clone()))
            .collect_vec()
    }
}

#[derive(Debug, Clone)]
struct CaveMap(Vec<&'static str>);

impl CaveMap {
    fn new() -> Self {
        CaveMap(Vec::new())
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
}

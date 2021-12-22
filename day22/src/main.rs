use std::{
    cmp::{max, min},
    collections::HashSet,
    ops::RangeInclusive,
};

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
}

// #[logging_timer::time]
fn part_1(input: &str) -> usize {
    let reboot_steps = parse_input(input)
        .into_iter()
        .filter_map(|i| {
            let (Instr::On([(xl, xu), (yl, yu), (zl, zu)])
            | Instr::Off([(xl, xu), (yl, yu), (zl, zu)])) = &i;

            if xl >= &-50 && xu <= &50 && yl >= &-50 && yu <= &50 && zl >= &-50 && zu <= &50 {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut cuboids = HashSet::new();
    for c in reboot_steps {
        match c {
            Instr::On([(xl, xu), (yl, yu), (zl, zu)]) => {
                for x in xl..=xu {
                    for y in yl..=yu {
                        for z in zl..=zu {
                            cuboids.insert(XYZ(x, y, z));
                        }
                    }
                }
            }
            Instr::Off([(xl, xu), (yl, yu), (zl, zu)]) => {
                for x in xl..=xu {
                    for y in yl..=yu {
                        for z in zl..=zu {
                            cuboids.remove(&XYZ(x, y, z));
                        }
                    }
                }
            }
        }
    }

    cuboids.len()
}

#[derive(PartialEq, Eq, Hash)]
struct XYZ(i32, i32, i32);

enum Instr {
    On([(i32, i32); 3]),
    Off([(i32, i32); 3]),
}

fn parse_input(input: &str) -> Vec<Instr> {
    let re = regex::Regex::new(
        r"(?P<action>on|off) x=(?P<xl>-?\d+)\.{2}(?P<xu>-?\d+),y=(?P<yl>-?\d+)\.{2}(?P<yu>-?\d+),z=(?P<zl>-?\d+)\.{2}(?P<zu>-?\d+)",
    )
    .unwrap();

    re.captures_iter(input)
        .map(|c| {
            let (action, xl, xu, yl, yu, zl, zu) = (
                c["action"].as_bytes(),
                c["xl"].parse::<i32>().unwrap(),
                c["xu"].parse::<i32>().unwrap(),
                c["yl"].parse::<i32>().unwrap(),
                c["yu"].parse::<i32>().unwrap(),
                c["zl"].parse::<i32>().unwrap(),
                c["zu"].parse::<i32>().unwrap(),
            );

            let (xl, xu) = (min(xl, xu), max(xl, xu));
            let (yl, yu) = (min(yl, yu), max(yl, yu));
            let (zl, zu) = (min(zl, zu), max(zl, zu));
            let xyz = [(xl, xu), (yl, yu), (zl, zu)];

            match action {
                b"on" => Instr::On(xyz),
                b"off" => Instr::Off(xyz),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 590784);
    }
}

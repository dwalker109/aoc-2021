#![feature(once_cell)]

use itertools::Itertools;
use std::{collections::HashMap, lazy::SyncLazy};

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[logging_timer::time]
fn part_1(input: &str) -> usize {
    let (mut p1_pos, mut p2_pos) = parse_input(input);

    let mut dd = (1..=100).cycle();
    let mut dd_rolls = 0u32;
    const ROLL_QTY: u8 = 3;

    let mut p1_score = 0usize;
    let mut p2_score = 0usize;

    loop {
        let p1_dd = dd.by_ref().take(ROLL_QTY as usize).sum::<usize>();
        dd_rolls += ROLL_QTY as u32;
        p1_pos = (((p1_pos as usize + p1_dd - 1) % 10) + 1) as u8;
        p1_score += p1_pos as usize;

        if p1_score >= 1000 {
            break;
        }

        let p2_dd = dd.by_ref().take(ROLL_QTY as usize).sum::<usize>();
        dd_rolls += ROLL_QTY as u32;
        p2_pos = (((p2_pos as usize + p2_dd - 1) % 10) + 1) as u8;
        p2_score += p2_pos as usize;

        if p2_score >= 1000 {
            break;
        }
    }

    std::cmp::min(p1_score, p2_score) * dd_rolls as usize
}

#[logging_timer::time]
fn part_2(input: &str) -> usize {
    let (p1_pos, p2_pos) = parse_input(input);
    let (p1, p2) = (
        Player(PlayerId::P1, p1_pos, 0),
        Player(PlayerId::P2, p2_pos, 0),
    );

    let wins = quantum_round(p1, p2, 1);

    std::cmp::max(wins.0, wins.1)
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
enum PlayerId {
    P1,
    P2,
}

#[derive(Clone, Copy)]
struct Player(PlayerId, u8, usize);

static UNIVERSE_ROLLS: SyncLazy<HashMap<u8, u8>> = SyncLazy::new(|| {
    (1..=3u8)
        .flat_map(|n1| (1..=3u8).flat_map(move |n2| (1..=3u8).map(move |n3| n1 + n2 + n3)))
        .sorted()
        .dedup_with_count()
        .map(|(freq, total)| (total, freq as u8))
        .collect::<HashMap<u8, u8>>()
});

fn quantum_round(p_curr: Player, p_other: Player, parent_freq: usize) -> (usize, usize) {
    let Player(player_id, pos, score) = p_curr;

    UNIVERSE_ROLLS
        .iter()
        .map(move |(num, freq)| {
            let pos = mv(&pos, num);
            let score = score + pos as usize;
            let p_curr = Player(player_id, pos, score);

            let freq = (*freq as usize) * parent_freq;

            if score >= 21 {
                match player_id {
                    PlayerId::P1 => (freq, 0),
                    PlayerId::P2 => (0, freq),
                }
            } else {
                quantum_round(p_other, p_curr, freq)
            }
        })
        .fold((0, 0), |acc, res| (acc.0 + res.0, acc.1 + res.1))
}

fn mv(pos: &u8, roll: &u8) -> u8 {
    ((pos + roll - 1) % 10) + 1
}

fn parse_input(input: &str) -> (u8, u8) {
    let starts = input
        .lines()
        .map(|l| l.chars().last().unwrap().to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();

    (starts[0], starts[1])
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, 739785);
    }

    #[test]
    fn part_2() {
        let r = super::part_2(INPUT);
        assert_eq!(r, 444356092776315);
    }
}

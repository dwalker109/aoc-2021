use crate::amphipod::Amphipod;
use crate::loc::Loc;
use crate::Cache;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Burrow {
    state: HashMap<(u8, u8), Loc>,
    num_moves: usize,
    expended: usize,
    finished: bool,
}

impl Burrow {
    fn get_state_key(&self) -> Vec<char> {
        let mut data = self.state.iter().collect::<Vec<_>>();
        data.sort_by_key(|(&k, _)| k);

        data.iter()
            .filter_map(|(_, l)| l.inner_ref().map_or(Some('-'), |&a| Some(a.into())))
            .collect::<Vec<char>>()
    }

    pub fn next_state(&self, leading_score: &mut usize, cache: &mut Cache) {
        if self.expended > *leading_score {
            return;
        }

        let key = self.get_state_key();
        let state_best = cache.get(&key).unwrap_or(&usize::MAX);
        if self.expended >= *state_best {
            return;
        } else {
            cache.insert(key, self.expended);
        }

        let sources = self
            .state
            .iter()
            .filter(|(_, l)| l.is_occupied())
            .filter(|(xy, _)| !self.loc_settled(xy))
            .collect::<Vec<_>>();
        let candidates = self
            .state
            .iter()
            .filter(|(_, l)| !l.is_occupied())
            .collect::<HashMap<_, _>>();

        for (&(source_x, source_y), source_loc) in sources {
            let amphipod = source_loc.inner_ref().unwrap();

            let target_xys = candidates
                .iter()
                .filter_map(|((x, y), target_loc)| {
                    let sibling = self.get_sibling(&(*x, *y));

                    // Always push to the bottom of the room - prevents endless churn
                    if *y == 1 && sibling.is_none() {
                        return None;
                    }

                    amphipod
                        .rules_check(source_loc, target_loc, sibling)
                        .then_some((*x, *y))
                })
                .collect::<Vec<_>>();

            // for each target, try to walk to the candidate loc.
            for (target_x, target_y) in target_xys {
                if let Ok(next) = self.travel(
                    amphipod,
                    leading_score,
                    (source_x, source_y),
                    (target_x, target_y),
                ) {
                    if !next.finished {
                        next.next_state(leading_score, cache);
                    }
                }
            }
        }
    }

    fn travel(
        &self,
        amphipod: &Amphipod,
        current_best: &mut usize,
        (from_x, from_y): (u8, u8),
        (to_x, to_y): (u8, u8),
    ) -> Result<Burrow, &'static str> {
        let mut next = (*self).clone();
        let (mut curr_x, mut curr_y) = (from_x, from_y);

        while curr_x != to_x || curr_y != to_y {
            if next.expended > *current_best || next.num_moves > 1000 {
                return Err("runaway train!");
            }

            // In corridor
            if curr_y == 0 && curr_x != to_x {
                let next_x = match to_x.cmp(&curr_x) {
                    Ordering::Less => curr_x - 1,
                    Ordering::Equal => unreachable!(),
                    Ordering::Greater => curr_x + 1,
                };

                if next.loc_blocked(&(next_x, curr_y)) {
                    return Err("blocked moving left/right!");
                }

                curr_x = next_x;
                next.expended += amphipod.cost() as usize;
            }

            // Aligned with room
            if curr_x == to_x && curr_y != to_y {
                let next_y = match to_y.cmp(&curr_y) {
                    Ordering::Less => curr_y - 1,
                    Ordering::Equal => unreachable!(),
                    Ordering::Greater => curr_y + 1,
                };

                if next.loc_blocked(&(curr_x, next_y)) {
                    return Err("blocked moving up/down!");
                }

                curr_y = next_y;
                next.expended += amphipod.cost() as usize;
            }

            // Need to leave room
            if curr_x != to_x && curr_y != 0 {
                let next_y = curr_y - 1;

                if next.loc_blocked(&(curr_x, next_y)) {
                    return Err("blocked moving up/down!");
                }

                curr_y = next_y;
                next.expended += amphipod.cost() as usize;
            }
        }

        next.state.entry((to_x, to_y)).and_modify(|l| match l {
            Loc::Open(_) => *l = Loc::Open(Some(*amphipod)),
            Loc::Doorway => unreachable!(),
            Loc::Room(required, _) => *l = Loc::Room(*required, Some(*amphipod)),
        });

        next.state.entry((from_x, from_y)).and_modify(|l| match l {
            Loc::Open(_) => *l = Loc::Open(None),
            Loc::Doorway => unreachable!(),
            Loc::Room(required, _) => *l = Loc::Room(*required, None),
        });

        next.num_moves += 1;

        next.finished = next
            .state
            .iter()
            .filter(|(_, l)| l.is_occupied())
            .all(|(_, l)| matches!(l, Loc::Room(required, Some(actual)) if required == actual));

        if next.finished && next.expended < *current_best {
            *current_best = next.expended
        }

        Ok(next)
    }

    pub fn loc_blocked(&self, loc: &(u8, u8)) -> bool {
        self.state
            .get(loc)
            .map(|l| l.is_occupied())
            .unwrap_or(false)
    }

    pub fn loc_settled(&self, &(x, y): &(u8, u8)) -> bool {
        match y {
            1 => {
                matches!(self.state.get(&(x,1)), Some(Loc::Room(required, Some(actual))) if required == actual)
                    && matches!(self.state.get(&(x,2)), Some(Loc::Room(required, Some(actual))) if required == actual)
            }
            2 => {
                matches!(self.state.get(&(x, 2)), Some(Loc::Room(required, Some(actual))) if required == actual)
            }
            _ => false,
        }
    }

    fn get_sibling(&self, &(x, y): &(u8, u8)) -> Option<&Amphipod> {
        matches!(self.state.get(&(x, y)), Some(Loc::Room(_, _)))
            .then(|| match y {
                1 => self.state.get(&(x, 2)).unwrap().inner_ref(),
                2 => self.state.get(&(x, 1)).unwrap().inner_ref(),
                _ => None,
            })
            .flatten()
    }
}

pub enum Part {
    One,
    Two,
}

impl From<(&str, Part)> for Burrow {
    fn from((raw, part): (&str, Part)) -> Self {
        let mut map = (0u8..=10u8)
            .map(|x| {
                (
                    (x, 0u8),
                    match x {
                        2 | 4 | 6 | 8 => Loc::Doorway,
                        _ => Loc::Open(None),
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        let amphipods = raw
            .chars()
            .filter_map(|c| {
                ['A', 'B', 'C', 'D']
                    .contains(&c)
                    .then(|| Amphipod::from(&c))
            })
            .collect::<Vec<_>>();

        map.insert((2, 1), Loc::Room(Amphipod::A, Some(amphipods[0])));
        map.insert((4, 1), Loc::Room(Amphipod::B, Some(amphipods[1])));
        map.insert((6, 1), Loc::Room(Amphipod::C, Some(amphipods[2])));
        map.insert((8, 1), Loc::Room(Amphipod::D, Some(amphipods[3])));
        map.insert((2, 2), Loc::Room(Amphipod::A, Some(amphipods[4])));
        map.insert((4, 2), Loc::Room(Amphipod::B, Some(amphipods[5])));
        map.insert((6, 2), Loc::Room(Amphipod::C, Some(amphipods[6])));
        map.insert((8, 2), Loc::Room(Amphipod::D, Some(amphipods[7])));

        Self {
            state: map,
            expended: 0,
            num_moves: 0,
            finished: false,
        }
    }
}

impl Display for Burrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = &self.state;
        writeln!(f, "#############").ok();
        writeln!(
            f,
            "#{}{}{}{}{}{}{}{}{}{}{}#",
            s.get(&(0, 0)).unwrap(),
            s.get(&(1, 0)).unwrap(),
            s.get(&(2, 0)).unwrap(),
            s.get(&(3, 0)).unwrap(),
            s.get(&(4, 0)).unwrap(),
            s.get(&(5, 0)).unwrap(),
            s.get(&(6, 0)).unwrap(),
            s.get(&(7, 0)).unwrap(),
            s.get(&(8, 0)).unwrap(),
            s.get(&(9, 0)).unwrap(),
            s.get(&(10, 0)).unwrap()
        )
        .ok();
        writeln!(
            f,
            "###{}#{}#{}#{}###",
            s.get(&(2, 1)).unwrap(),
            s.get(&(4, 1)).unwrap(),
            s.get(&(6, 1)).unwrap(),
            s.get(&(8, 1)).unwrap()
        )
        .ok();
        writeln!(
            f,
            "###{}#{}#{}#{}###",
            s.get(&(2, 2)).unwrap(),
            s.get(&(4, 2)).unwrap(),
            s.get(&(6, 2)).unwrap(),
            s.get(&(8, 2)).unwrap()
        )
        .ok();
        writeln!(f, "  #########  ").ok();

        Ok(())
    }
}

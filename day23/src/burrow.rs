use crate::amphipod::Amphipod;
use crate::loc::Loc;
use crate::Cache;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Burrow {
    state: HashMap<(u8, u8), Loc>,
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

    pub fn next_state(&self, part: &Part, leading_score: &mut usize, cache: &mut Cache) {
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
            .filter(|(_, l)| l.is_occupied() && !l.is_settled())
            .collect::<Vec<_>>();
        let candidates = self
            .state
            .iter()
            .filter(|(_, l)| !l.is_occupied())
            .collect::<HashMap<_, _>>();

        for (&(source_x, source_y), source_loc) in sources {
            let ap = source_loc.inner_ref().unwrap();

            let target_xys = candidates
                .iter()
                .filter_map(|((x, y), target_loc)| {
                    let siblings = self.get_siblings(&(*x, *y));

                    // Always push to the bottom of the room - prevents endless churn
                    if *y != 0 && *y + siblings.len() as u8 != part.room_depth() {
                        return None;
                    }

                    ap.rules_check(source_loc, target_loc, siblings)
                        .then_some((*x, *y))
                })
                .collect::<Vec<_>>();

            // for each target, try to walk to the candidate loc.
            for (target_x, target_y) in target_xys {
                if let Ok(next) = self.travel(
                    ap,
                    leading_score,
                    (source_x, source_y),
                    (target_x, target_y),
                ) {
                    if !next.finished {
                        next.next_state(part, leading_score, cache);
                    }
                }
            }
        }
    }

    fn travel(
        &self,
        ap: &Amphipod,
        current_best: &mut usize,
        (from_x, from_y): (u8, u8),
        (to_x, to_y): (u8, u8),
    ) -> Result<Burrow, &'static str> {
        let mut next = (*self).clone();
        let (mut curr_x, mut curr_y) = (from_x, from_y);

        while curr_x != to_x || curr_y != to_y {
            if next.expended > *current_best {
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
                next.expended += ap.cost() as usize;
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
                next.expended += ap.cost() as usize;
            }

            // Need to leave room
            if curr_x != to_x && curr_y != 0 {
                let next_y = curr_y - 1;

                if next.loc_blocked(&(curr_x, next_y)) {
                    return Err("blocked moving up/down!");
                }

                curr_y = next_y;
                next.expended += ap.cost() as usize;
            }
        }

        next.state.entry((to_x, to_y)).and_modify(|l| match l {
            Loc::Open(_) => *l = Loc::Open(Some(*ap)),
            Loc::Doorway => unreachable!(),
            Loc::Room(required, _, _) => {
                let siblings = self.get_siblings(&(to_x, to_y));
                let settled = siblings.is_empty() || siblings.iter().all(|&s| s == required);
                *l = Loc::Room(*required, Some(*ap), settled)
            }
        });

        next.state.entry((from_x, from_y)).and_modify(|l| match l {
            Loc::Open(_) => *l = Loc::Open(None),
            Loc::Doorway => unreachable!(),
            Loc::Room(required, _, _) => *l = Loc::Room(*required, None, false),
        });

        next.finished =
            next.state.iter().filter(|(_, l)| l.is_occupied()).all(
                |(_, l)| matches!(l, Loc::Room(required, Some(actual), _) if required == actual),
            );

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

    fn get_siblings(&self, &(subj_x, subj_y): &(u8, u8)) -> Vec<&Amphipod> {
        (1..=4)
            .filter(|sib_y| *sib_y != subj_y)
            .filter_map(|y| self.state.get(&(subj_x, y)).and_then(|l| l.inner_ref()))
            .collect()
    }
}

pub enum Part {
    One,
    Two,
}

impl Part {
    fn room_depth(&self) -> u8 {
        match self {
            Part::One => 2,
            Part::Two => 4,
        }
    }
}

impl From<(&str, &Part)> for Burrow {
    fn from((raw, part): (&str, &Part)) -> Self {
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

        let aps = raw
            .chars()
            .filter_map(|c| {
                ['A', 'B', 'C', 'D']
                    .contains(&c)
                    .then(|| Amphipod::from(&c))
            })
            .collect::<Vec<_>>();

        let last_row = match part {
            Part::One => 2,
            Part::Two => 4,
        };

        map.insert((2, 1), Loc::Room(Amphipod::A, Some(aps[0]), false));
        map.insert((4, 1), Loc::Room(Amphipod::B, Some(aps[1]), false));
        map.insert((6, 1), Loc::Room(Amphipod::C, Some(aps[2]), false));
        map.insert((8, 1), Loc::Room(Amphipod::D, Some(aps[3]), false));
        map.insert((2, last_row), Loc::Room(Amphipod::A, Some(aps[4]), false));
        map.insert((4, last_row), Loc::Room(Amphipod::B, Some(aps[5]), false));
        map.insert((6, last_row), Loc::Room(Amphipod::C, Some(aps[6]), false));
        map.insert((8, last_row), Loc::Room(Amphipod::D, Some(aps[7]), false));

        if matches!(part, Part::Two) {
            map.insert((2, 2), Loc::Room(Amphipod::A, Some(Amphipod::D), false));
            map.insert((4, 2), Loc::Room(Amphipod::B, Some(Amphipod::C), false));
            map.insert((6, 2), Loc::Room(Amphipod::C, Some(Amphipod::B), false));
            map.insert((8, 2), Loc::Room(Amphipod::D, Some(Amphipod::A), false));
            map.insert((2, 3), Loc::Room(Amphipod::A, Some(Amphipod::D), false));
            map.insert((4, 3), Loc::Room(Amphipod::B, Some(Amphipod::B), false));
            map.insert((6, 3), Loc::Room(Amphipod::C, Some(Amphipod::A), false));
            map.insert((8, 3), Loc::Room(Amphipod::D, Some(Amphipod::C), false));
        }

        let mut burrow = Self {
            state: map,
            expended: 0,
            finished: false,
        };

        // Horrid but we do it once, :shrug
        let burrow_clone = burrow.clone();

        for (xy, l) in burrow.state.iter_mut() {
            if let Loc::Room(required, Some(ap), _) = l {
                let siblings = burrow_clone.get_siblings(&xy);
                let settled = *ap == *required
                    && (siblings.is_empty() || siblings.iter().all(|&&s| s == *required));
                *l = Loc::Room(*required, Some(*ap), settled);
            }
        }

        burrow
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

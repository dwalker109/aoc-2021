use crate::loc::Loc;
use std::fmt::{Display, Formatter, Write};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Amphipod {
    A,
    B,
    C,
    D,
}

impl From<&char> for Amphipod {
    fn from(raw: &char) -> Self {
        match raw {
            'A' => Self::A,
            'B' => Self::B,
            'C' => Self::C,
            'D' => Self::D,
            _ => panic!(),
        }
    }
}

impl From<Amphipod> for char {
    fn from(ap: Amphipod) -> char {
        match ap {
            Amphipod::A => 'A',
            Amphipod::B => 'B',
            Amphipod::C => 'C',
            Amphipod::D => 'D',
        }
    }
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char((*self).into())
    }
}

impl Amphipod {
    pub fn cost(&self) -> u16 {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }

    pub fn rules_check(&self, from: &Loc, to: &Loc, sibling: Option<&Amphipod>) -> bool {
        match from {
            Loc::Open(_) => match to {
                Loc::Open(_) | Loc::Doorway => false,
                Loc::Room(required, _) => {
                    self == required
                        && (sibling.is_none() || sibling.is_some_and(|s| s == required))
                }
            },
            Loc::Doorway => {
                unreachable!();
            }
            Loc::Room(_, _) => match to {
                Loc::Open(occupant) => occupant.is_none(),
                Loc::Doorway => false,
                Loc::Room(required, _) => {
                    self == required
                        && (sibling.is_none() || sibling.is_some_and(|s| s == required))
                }
            },
        }
    }
}

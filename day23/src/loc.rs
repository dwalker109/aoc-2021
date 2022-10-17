use crate::amphipod::Amphipod;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Loc {
    Open(Option<Amphipod>),
    Doorway,
    Room(Amphipod, Option<Amphipod>),
}

impl Loc {
    pub fn is_occupied(&self) -> bool {
        matches!(self, Loc::Open(Some(_)) | Loc::Room(_, Some(_)))
    }

    pub fn inner_ref(&self) -> Option<&Amphipod> {
        match self {
            Loc::Open(Some(ap)) | Loc::Room(_, Some(ap)) => Some(ap),
            _ => None,
        }
    }
}

impl Display for Loc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ap = self
            .inner_ref()
            .map(|a| a.to_string())
            .unwrap_or(" ".into());
        f.write_str(&ap)
    }
}

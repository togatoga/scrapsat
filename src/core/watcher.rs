use crate::{clause::alloc::CRef, collections::idxvec::LitVec, types::lit::Lit};

/// `blocker` is clause[0] or clause[1].
/// `cref` points that a clause that is watched.
pub struct Watch {
    blocker: Lit,
    cref: CRef,
}

impl Watch {
    pub fn new(cref: CRef, blocker: Lit) -> Watch {
        Watch { cref, blocker }
    }
}

pub struct Watches {
    watchers: LitVec<Vec<Watch>>,
}

impl Watches {
    pub fn new() -> Watches {
        Watches {
            watchers: LitVec::new(),
        }
    }
}

use crate::{clause::alloc::CRef, collections::idxvec::LitVec, types::lit::Lit};

/// `blocker` is clause[0] or clause[1].
/// `cref` points that a clause that is watched.
pub struct Watch {
    pub blocker: Lit,
    pub cref: CRef,
}

impl Watch {
    pub fn new(cref: CRef, blocker: Lit) -> Watch {
        Watch { cref, blocker }
    }
}

pub struct Watchers {
    watchers: LitVec<Vec<Watch>>,
}

impl Watchers {
    pub fn new() -> Watchers {
        Watchers {
            watchers: LitVec::new(),
        }
    }
    pub fn as_mut_ptr(&mut self) -> *mut LitVec<Vec<Watch>> {
        &mut self.watchers as *mut LitVec<Vec<Watch>>
    }
    pub fn lookup_mut(&mut self, lit: Lit) -> &mut Vec<Watch> {
        &mut self.watchers[lit]
    }
}

use crate::types::lit::Lit;

use super::alloc::{CRef, ClauseAllocator};

pub struct ClauseDB {
    ca: ClauseAllocator,
    /// original clauses
    clauses: Vec<CRef>,
    /// learnt clauses
    learnts: Vec<CRef>,
}

impl ClauseDB {
    pub fn new() -> ClauseDB {
        ClauseDB {
            ca: ClauseAllocator::new(),
            clauses: Vec::new(),
            learnts: Vec::new(),
        }
    }

    pub fn alloc(&mut self, lits: &[Lit], learnt: bool) -> CRef {
        let cref = self.ca.alloc(lits, learnt);
        if learnt {
            self.learnts.push(cref);
        } else {
            self.clauses.push(cref);
        }
        cref
    }
    pub fn free(&mut self, cref: CRef) {
        self.ca.free(cref);
    }
}

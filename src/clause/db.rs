use crate::types::lit::Lit;

use super::{
    alloc::{CRef, ClauseAllocator},
    Clause,
};

pub struct ClauseDB {
    ca: ClauseAllocator,
    /// original clauses
    clauses: Vec<CRef>,
    /// learnt clauses
    learnts: Vec<CRef>,
}

impl Default for ClauseDB {
    fn default() -> Self {
        ClauseDB {
            ca: ClauseAllocator::default(),
            clauses: Vec::default(),
            learnts: Vec::default(),
        }
    }
}

impl ClauseDB {
    pub fn new() -> ClauseDB {
        ClauseDB {
            ca: ClauseAllocator::new(),
            clauses: Vec::new(),
            learnts: Vec::new(),
        }
    }

    pub fn get_mut(&mut self, cref: CRef) -> Clause {
        self.ca.get_mut(cref)
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

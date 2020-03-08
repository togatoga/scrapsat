use crate::clause::{Clause, ClauseAllocator, ClauseRef};
use crate::index_vec::{Idx, IdxVec, LitVec};
use crate::lit::Lit;
use crate::{var_to_lit, Var};

pub struct Watcher {
    cref: ClauseRef,
    blocker: Lit,
}

impl Watcher {
    pub fn new(cref: ClauseRef, blocker: Lit) -> Watcher {
        Watcher { cref, blocker }
    }
}

pub struct Watches {
    watches: LitVec<Vec<Watcher>>,
    dirty: LitVec<bool>,
    dirties: Vec<Lit>,
}

impl Watches {
    pub fn new() -> Watches {
        Watches {
            watches: LitVec::new(),
            dirty: LitVec::new(),
            dirties: Vec::new(),
        }
    }
    pub fn init_var(&mut self, v: Var) {
        self.watches.init(Lit::new(v, false));
        self.watches.init(Lit::new(v, true));
        self.dirty.init(Lit::new(v, false));
        self.dirty.init(Lit::new(v, true));
    }
    pub fn watch_clause(&mut self, c: &Clause, cref: ClauseRef) {
        debug_assert!(c.len() >= 2);
        self.watches[!c[0]].push(Watcher::new(cref, c[1]));
        self.watches[!c[1]].push(Watcher::new(cref, c[0]));
    }

    pub fn clean_all(&mut self, ca: &ClauseAllocator) {
        for x in self.dirties.iter() {
            if self.dirty[*x] {
                self.watches[*x].retain(|w| !ca.clause(w.cref).header.is_deleted());
            }
        }
        self.dirties.clear();
    }

    pub fn smudge(&mut self, idx: Lit) {
        if !self.dirty[idx] {
            self.dirty[idx] = true;
            self.dirties.push(idx);
        }
    }

    pub fn unwatch_clause(&mut self, c: &Clause, cref: ClauseRef) {
        self.watches[!c[0]].retain(|w| w.cref != cref);
        self.watches[!c[1]].retain(|w| w.cref != cref);
    }
    pub fn unwatch_clause_lazy(&mut self, c: &Clause) {
        debug_assert!(c.len() >= 2);
        self.smudge(!c[0]);
        self.smudge(!c[1]);
    }
}

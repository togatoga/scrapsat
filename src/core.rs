use assign::AssignTrail;
use data::VarData;
use watcher::{Watch, Watchers};

use crate::{
    clause::{alloc::CRef, db::ClauseDB},
    types::{bool::LitBool, lit::Lit},
};

mod assign;
mod data;
mod watcher;

pub enum SatResult {
    Sat,
    Unsat,
    Unknown,
}

impl Default for SatResult {
    fn default() -> Self {
        SatResult::Unknown
    }
}

pub struct Solver {
    db: ClauseDB,
    vardata: VarData,
    watches: Watchers,
    result: SatResult,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            db: ClauseDB::new(),
            vardata: VarData::new(),
            watches: Watchers::new(),
            result: SatResult::Unknown,
        }
    }

    fn simplify_clause(&self, lits: &[Lit]) -> (bool, Vec<Lit>) {
        let mut lits = lits.to_vec();
        lits.sort();
        let mut len = 0;
        for i in 0..lits.len() {
            let mut remove = false;
            if i >= 1 {
                // x0 v !x0 means a clause is already satisfied.
                // you don't need to add it.
                if lits[i] == !lits[i - 1] {
                    return (true, lits);
                }
                // x0 v x0 duplicated
                if lits[i] == lits[i - 1] {
                    remove = true;
                }
            }
            //already assigned
            match self.vardata.eval(lits[i]) {
                LitBool::True => {
                    // a clause is already satisfied. You don't need to add it.
                    return (true, lits);
                }
                LitBool::False => {
                    // a literal is already false. You can remove it from a clause.
                    remove = true;
                }
                _ => {}
            }

            if !remove {
                lits[len] = lits[i];
                len += 1;
            }
        }
        lits.truncate(len);
        (false, lits)
    }

    pub fn propagate(&mut self) -> CRef {
        while self.vardata.trail.peekable() {
            let p = self.vardata.trail.peek();
            self.vardata.trail.advance();
            let mut watchers_ptr = self.watches.as_mut_ptr();
            let mut ws = self.watches.lookup_mut(p);
            let mut idx = 0;

            'next_clause: while idx < ws.len() {
                let blocker = ws[idx].blocker;
                if self.vardata.eval(blocker) == LitBool::True {
                    idx += 1;
                    continue;
                }
                let cref = ws[idx].cref;
                let mut clause = self.db.get_mut(cref);
                debug_assert!(!clause.deleted());
                debug_assert!(clause[0] == !p || clause[1] == !p);
                // make sure that clause[1] is a false literal.
                if clause[0] == !p {
                    clause.swap(0, 1);
                }
                let first = clause[0];
                let w = Watch::new(cref, first);
                // already satisfied
                if first != blocker && self.vardata.eval(first) == LitBool::True {
                    debug_assert!(first != clause[1]);
                    ws[idx] = w;
                    idx += 1;
                    continue 'next_clause;
                }

                for k in 2..clause.len() {
                    let lit = clause[k];
                    if self.vardata.eval(lit) != LitBool::False {
                        clause.swap(1, k);
                        ws.swap_remove(idx);
                        unsafe { &mut (*watchers_ptr)[!clause[1]].push(w) };
                        continue 'next_clause;
                    }
                }
                ws[idx] = w;
                if self.vardata.eval(first) == LitBool::False {
                    return cref;
                } else {
                    self.vardata.enqueue(first, cref);
                }
                idx += 1;
            }
        }

        CRef::UNDEF
    }
    pub fn add_clause(&mut self, lits: &[Lit]) {
        debug_assert!(self.vardata.trail.decision_level() == 0);
        lits.iter().for_each(|lit| {
            while lit.var().val() >= self.vardata.num_var() as u32 {
                self.vardata.new_var();
            }
        });

        let (skip, lits) = self.simplify_clause(lits);

        if skip {
            return;
        }

        if lits.is_empty() {
            self.result = SatResult::Unsat;
            return;
        } else if lits.len() == 1 {
            if self.vardata.eval(lits[0]) == LitBool::False {
                self.result = SatResult::Unsat;
                return;
            }
            self.vardata.enqueue(lits[0], CRef::UNDEF);
        } else {
        }
    }
}

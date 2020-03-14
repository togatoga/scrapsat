use crate::assignments::Assignment;
use crate::clause::{ClauseAllocator, ClauseRef};
use crate::index_vec::Idx;
use crate::lit::Lit;
use crate::watcher::{Watcher, Watches};

pub struct Solver {
    assignment: Assignment,
    watches: Watches,
    ca: ClauseAllocator,
    clauses: Vec<ClauseRef>,        //vector of problem clauses.
    learnt_clauses: Vec<ClauseRef>, //vector of problem clauses.
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

pub enum AddClauseResult {
    UnSAT,
    ClauseAlreadySatisfied,
    None,
}
impl Solver {
    pub fn new() -> Solver {
        Solver {
            assignment: Assignment::new(),
            watches: Watches::new(),
            ca: ClauseAllocator::new(),
            clauses: Vec::new(),
            learnt_clauses: Vec::new(),
        }
    }

    // n_var returns the current number of variables.
    pub fn n_var(&self) -> usize {
        self.assignment.n_var()
    }

    pub fn add_clause(&mut self, lits: &[Lit]) -> AddClauseResult {
        //Reserve the space of variables
        lits.iter().for_each(|lit| {
            let var = lit.var();
            while var.idx() >= self.assignment.n_var() {
                self.assignment.new_var();
                self.watches.init_var(var);
            }
        });
        let lits: Vec<Lit> = {
            let mut lits = lits.to_vec();
            lits.sort();
            lits.dedup();
            lits.retain(|&lit| !self.assignment.is_assigned_false(lit));
            let mut prev = None;
            for &lit in lits.iter() {
                if self.assignment.is_assigned_true(lit) || prev == Some(!lit) {
                    return AddClauseResult::ClauseAlreadySatisfied;
                }
                prev = Some(lit);
            }

            lits
        };
        println!("{:?}", lits);
        match &lits[..] {
            [] => {
                //UNSAT
                AddClauseResult::UnSAT
            }
            [unit] => {
                self.assignment.assign_true(*unit, None);
                match self.propagate() {
                    Some(_) => AddClauseResult::UnSAT,
                    None => AddClauseResult::None,
                }
            }
            lits => {
                let cref = self.ca.alloc(lits);
                self.clauses.push(cref);
                self.watches.watch_clause(self.ca.clause(cref), cref);
                AddClauseResult::None
            }
        }
    }

    pub fn propagate(&mut self) -> Option<ClauseRef> {
        self.watches.clean_all(&self.ca);
        while let Some(p) = self.assignment.pop_front_trail() {
            let not_p = !p;
            let mut tail_idx = 0;
            let end_idx = self.watches.get_watches(p).len();

            'next_watch: for idx in 0..end_idx {
                let (w_cref, w_blocker) = {
                    let watcher = self.watches.get_watches(p)[idx];
                    (watcher.cref, watcher.blocker)
                };
                // Try not to avoid inspecting the clause:
                if self.assignment.is_assigned_true(w_blocker) {
                    *self.watches.get_watcher_mut(p, tail_idx) = Watcher {
                        cref: w_cref,
                        blocker: w_blocker,
                    };
                    tail_idx += 1;
                    continue 'next_watch;
                }
                let clause = self.ca.clause_mut(w_cref);

                // Make sure the false literal is data[1]
                if clause[0] == not_p {
                    clause[0] = clause[1];
                    clause[1] = not_p;
                }
                debug_assert_eq!(clause[1], not_p);

                //If 0th watch is true, then clause is already satisfied.
                let first = clause[0];
                let cw = Watcher {
                    cref: w_cref,
                    blocker: first,
                };
                if cw.blocker != w_blocker && self.assignment.is_assigned_true(cw.blocker) {
                    *self.watches.get_watcher_mut(p, tail_idx) = cw;
                    tail_idx += 1;
                    continue 'next_watch;
                }

                //Look for new watch
                for k in 2..clause.len() {
                    //true or unassigned
                    if !self.assignment.is_assigned_false(clause[k]) {
                        debug_assert_eq!(clause[1], not_p);
                        clause[1] = clause[k];
                        clause[k] = not_p;
                        self.watches.get_watches_mut(!clause[1]).push(cw);
                        continue 'next_watch;
                    }
                }
                *self.watches.get_watcher_mut(p, tail_idx) = cw;
                tail_idx += 1;
                //Did not find watch -- clause is unit under assignment
                if self.assignment.is_assigned_false(cw.blocker) {
                    // Copy the remaining watches
                    let mut tmp_idx = idx;
                    while tmp_idx < end_idx {
                        *self.watches.get_watcher_mut(p, tail_idx) =
                            self.watches.get_watches(p)[tmp_idx];
                        tmp_idx += 1;
                        tail_idx += 1;
                    }
                    //Cancel the rest of trail.
                    self.assignment.head = self.assignment.trail.len();
                    self.watches.get_watches_mut(p).truncate(tail_idx);
                    return Some(cw.cref);
                } else {
                    self.assignment.assign_true(cw.blocker, Some(cw.cref));
                }
            }
            self.watches.get_watches_mut(p).truncate(tail_idx);
        }
        None
    }
}

use crate::assignments::Assignment;
use crate::clause::{ClauseAllocator, ClauseRef};
use crate::index_vec::Idx;
use crate::lit::{Lit, LitBool};
use crate::watcher::{Watcher, Watches};

pub struct Solver {
    pub assignment: Assignment,
    watches: Watches,
    ca: ClauseAllocator,
    clauses: Vec<ClauseRef>,        //vector of problem clauses
    learnt_clauses: Vec<ClauseRef>, //vector of problem clauses
    ok: bool, // If FALSE, the constraints are already unsatisfiable. No part of the solver state may be used!
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
            ok: true,
        }
    }

    /*_________________________________________________________________________________________________
    |
    |  simplify : [void]  ->  [bool]
    |
    |  Description:
    |    Simplify the clause database according to the current top-level assigment. Currently, the only
    |    thing done here is the removal of satisfied clauses, but more things can be put here.
    |________________________________________________________________________________________________@*/
    pub fn simplify(&mut self) -> bool {
        todo!()
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

        match &lits[..] {
            [] => {
                //UNSAT
                AddClauseResult::UnSAT
            }
            [unit] => {
                self.assignment.assign_true(*unit, None);
                match self.propagate() {
                    Some(_) => {
                        self.ok = false;
                        AddClauseResult::UnSAT
                    }
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
            let mut head_idx = 0;
            let mut tail_idx = 0;
            let end_idx = self.watches.get_watches(p).len();
            //eprintln!("{:?}", end_idx);
            'next_watch: while head_idx < end_idx {
                assert!(head_idx < end_idx);
                assert!(tail_idx < end_idx);
                let (w_cref, w_blocker) = {
                    let watcher = self.watches.get_watches(p)[head_idx];
                    (watcher.cref, watcher.blocker)
                };
                // Try not to avoid inspecting the clause:
                if self.assignment.is_assigned_true(w_blocker) {
                    *self.watches.get_watcher_mut(p, tail_idx) =
                        self.watches.get_watches(p)[head_idx];
                    head_idx += 1;
                    tail_idx += 1;
                    continue 'next_watch;
                }
                let clause = self.ca.clause_mut(w_cref);
                head_idx += 1;

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
                    while head_idx < end_idx {
                        *self.watches.get_watcher_mut(p, tail_idx) =
                            self.watches.get_watches(p)[head_idx];
                        head_idx += 1;
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

    pub fn solve_limited(&mut self) -> LitBool {
        //dbg!(self.ok);
        if !self.ok {
            return LitBool::False;
        }
        let mut status = LitBool::Undef;

        let luby = |y: f64, mut x: i32| -> f64 {
            /*
             Finite subsequences of the Luby-sequence:

             0: 1
             1: 1 1 2
             2: 1 1 2 1 1 2 4
             3: 1 1 2 1 1 2 4 1 1 2 1 1 2 4 8
             ...


            */
            let mut size = 1;
            let mut seq = 0;
            while size < x + 1 {
                seq += 1;
                size = 2 * size + 1;
            }
            while size - 1 != x {
                size = (size - 1) >> 1;
                seq -= 1;
                x %= size;
            }
            let _ = seq;
            y.powi(x)
        };
        let curr_restarts = 0;
        let restart_inc = 1.5;
        let restart_first = 100;
        while status == LitBool::Undef {
            let rest_base = luby(restart_inc, curr_restarts);
            let nof_conflicts = (rest_base * restart_first as f64) as i32;
            status = self.search(nof_conflicts);
        }
        status
    }

    pub fn analyze(&mut self, confl: ClauseRef, learnt_clause: &mut Vec<Lit>) -> usize {
        let mut path_c = 0;
        let mut p = None;
        learnt_clause.clear();
        learnt_clause.push(Lit::default()); // (leave room for the asserting literal)
        let mut index = self.assignment.trail.len() - 1;
        let mut confl = Some(confl);

        loop {
            //dbg!(path_c, confl);
            assert!(confl.is_some());

            let c = self.ca.clause_mut(confl.unwrap());
            let j = if p.is_none() { 0usize } else { 1usize };

            for &q in c.lits.iter().skip(j) {
                if !self.assignment.seen(q) && self.assignment.decision_level(q) > 0 {
                    self.assignment.check(q);
                    if self.assignment.decision_level(q) >= self.assignment.current_decision_level()
                    {
                        path_c += 1;
                    } else {
                        learnt_clause.push(q);
                    }
                }
            }
            // Select next clause to look at
            //dbg!(path_c, index);

            while !self.assignment.seen(self.assignment.trail[index]) {
                index -= 1;
            }
            p = Some(self.assignment.trail[index]);
            confl = self.assignment.reason(p.unwrap());
            self.assignment.uncheck(p.unwrap());
            path_c -= 1;
            if path_c <= 0 {
                break;
            }
        }
        learnt_clause[0] = !p.unwrap();

        let analyze_clear = learnt_clause.clone();
        //TODO
        //Simplify

        // Find correct backtrack level

        let backtrack_level = if learnt_clause.len() == 1 {
            0
        } else {
            let mut max_idx = 1;
            let mut min_level = self.assignment.decision_level(learnt_clause[1]);
            for (idx, &p) in learnt_clause.iter().enumerate().skip(2) {
                if self.assignment.decision_level(p) > min_level {
                    min_level = self.assignment.decision_level(p);
                    max_idx = idx;
                }
            }
            // Swap-in this literal at index 1:
            learnt_clause.swap(max_idx, 1);
            min_level
        };
        for elem in analyze_clear {
            self.assignment.uncheck(elem);
        }

        backtrack_level
    }

    /*_________________________________________________________________________________________________
    |
    |  search : (nof_conflicts : int) (params : const SearchParams&)  ->  [lbool]
    |
    |  Description:
    |    Search for a model the specified number of conflicts.
    |    NOTE! Use negative value for 'nof_conflicts' indicate infinity.
    |
    |  Output:
    |    'l_True' if a partial assigment that is consistent with respect to the clauseset is found. If
    |    all variables are decision variables, this means that the clause set is satisfiable. 'l_False'
    |    if the clause set is unsatisfiable. 'l_Undef' if the bound on number of conflicts is reached.
    |________________________________________________________________________________________________@*/
    fn search(&mut self, nof_conflicts: i32) -> LitBool {
        debug_assert!(self.ok);
        let mut learnt_clause: Vec<Lit> = vec![];
        let mut conflict_cnt = 0;

        loop {
            let confl = self.propagate();
            //dbg!(confl);
            if let Some(confl) = confl {
                //CONFLICT
                if self.assignment.current_decision_level() == 0 {
                    return LitBool::False;
                }
                learnt_clause.clear();
                //analyze
                let backtrack_level = self.analyze(confl, &mut learnt_clause);
                self.assignment.cancel_until(backtrack_level);
                conflict_cnt += 1;

                //cancelUntil
                if learnt_clause.len() == 1 {
                    self.assignment.assign_true(learnt_clause[0], None);
                } else {
                    let cr = self.ca.alloc(&learnt_clause);
                    self.learnt_clauses.push(cr);
                    self.watches.watch_clause(self.ca.clause(cr), cr);
                    self.assignment.assign_true(learnt_clause[0], Some(cr));
                }
            } else {
                //NO CONFLICT
                if conflict_cnt >= nof_conflicts {
                    self.assignment.cancel_until(0);
                    break;
                }
                //eprintln!("{}", self.assignment.current_decision_level());

                //dbg!(self.assignment.current_decision_level());
                // if self.assignment.current_decision_level() == 0 {
                //     return LitBool::False;
                // }
                // TODO
                // Reduce the set of learnt clauses:

                // New variable decision:
                if let Some(next) = self.assignment.pick_bracnh_lit() {
                    // Increase decsion level and enqueue 'next'
                    self.assignment.new_decision_level();
                    self.assignment.assign_true(next, None);
                } else {
                    return LitBool::True;
                }
            }
        }

        LitBool::Undef
    }
}

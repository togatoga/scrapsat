use data::VarData;
use watcher::{Watch, Watchers};

use crate::{
    clause::{alloc::CRef, db::ClauseDB},
    types::{bool::LitBool, lit::Lit, var},
};

mod analyzer;
mod assign;
mod data;
mod watcher;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    /// check clauses if a propagation or conflict happens.
    watches: Watchers,
    result: SatResult,
    pub models: Vec<LitBool>,
}

impl Default for Solver {
    fn default() -> Self {
        Solver {
            db: ClauseDB::new(),
            vardata: VarData::new(),
            watches: Watchers::new(),
            result: SatResult::Unknown,
            models: Vec::new(),
        }
    }
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            db: ClauseDB::new(),
            vardata: VarData::new(),
            watches: Watchers::new(),
            result: SatResult::Unknown,
            models: Vec::new(),
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
            let watchers_ptr = self.watches.as_mut_ptr();
            let ws = self.watches.lookup_mut(p);
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
    fn new_var(&mut self) {
        self.vardata.new_var();
        self.watches.new_var();
    }

    pub fn add_clause(&mut self, lits: &[Lit]) {
        debug_assert!(self.vardata.trail.decision_level() == 0);
        lits.iter().for_each(|lit| {
            while lit.var().val() >= self.vardata.num_var() as u32 {
                self.new_var();
            }
        });

        let (skip, lits) = self.simplify_clause(lits);

        if skip {
            return;
        }

        if lits.is_empty() {
            self.result = SatResult::Unsat;
        } else if lits.len() == 1 {
            if self.vardata.eval(lits[0]) == LitBool::False {
                self.result = SatResult::Unsat;
                return;
            }
            self.vardata.enqueue(lits[0], CRef::UNDEF);
            if self.propagate() != CRef::UNDEF {
                self.result = SatResult::Unsat;
            }
        } else {
            let cref = self.db.alloc(&lits, false);
            self.watches.watch(&lits, cref);
        }
    }
    fn analyze(&mut self, confl: CRef) -> u32 {
        debug_assert!(confl != CRef::UNDEF);
        let decision_level = self.vardata.trail.decision_level();
        self.vardata.analyzer.learnt_clause.clear();
        self.vardata.analyzer.learnt_clause.push(Lit::default());

        let mut counter = 0;
        {
            let clause = self.db.get_mut(confl);
            debug_assert!(!clause.deleted());
            for &p in clause.iter() {
                debug_assert!(self.vardata.eval(p) != LitBool::UnDef);
                let var = p.var();
                self.vardata.analyzer.seen[var] = true;
                if self.vardata.level(var) < decision_level {
                    self.vardata.analyzer.learnt_clause.push(p);
                } else {
                    counter += 1;
                }
            }
        }
        // Traverse an implication graph to 1-UIP(unique implication point)
        let first_uip = {
            let mut p = Lit::UNDEF;
            for &lit in self.vardata.trail.stack.iter().rev() {
                // skip a variable that isn't checked.
                if !self.vardata.analyzer.seen[lit.var()] {
                    continue;
                }
                self.vardata.analyzer.seen[lit.var()] = true;
                counter -= 1;
                if counter <= 0 {
                    p = lit;
                    break;
                }
                let reason = self.vardata.reason(lit.var());
                let clause = self.db.get_mut(reason);
                for &q in clause.iter().skip(1) {
                    if self.vardata.analyzer.seen[q.var()] {
                        continue;
                    }
                    self.vardata.analyzer.seen[q.var()] = true;
                    if self.vardata.level(q.var()) < decision_level {
                        self.vardata.analyzer.learnt_clause.push(q);
                    } else {
                        counter += 1;
                    }
                }
            }
            p
        };
        debug_assert!(first_uip != Lit::UNDEF);
        self.vardata.analyzer.learnt_clause[0] = !first_uip;
        self.vardata
            .analyzer
            .analyze_toclear
            .clone_from(&self.vardata.analyzer.learnt_clause);

        let backtrack_level = if self.vardata.analyzer.learnt_clause.len() == 1 {
            0
        } else {
            let mut idx = 1;
            let mut max_level = self
                .vardata
                .level(self.vardata.analyzer.learnt_clause[1].var());
            for (i, lit) in self
                .vardata
                .analyzer
                .learnt_clause
                .iter()
                .enumerate()
                .skip(2)
            {
                if self.vardata.level(lit.var()) > max_level {
                    max_level = self.vardata.level(lit.var());
                    idx = i;
                }
            }
            self.vardata.analyzer.learnt_clause.swap(1, idx);
            max_level
        };

        // clear seen
        for lit in self.vardata.analyzer.analyze_toclear.iter() {
            self.vardata.analyzer.seen[lit.var()] = false;
        }
        backtrack_level
    }
    fn search(&mut self) -> SatResult {
        loop {
            let confl = self.propagate();
            // conflict
            if confl != CRef::UNDEF {
                if self.vardata.trail.decision_level() == 0 {
                    self.result = SatResult::Unsat;
                    return SatResult::Unsat;
                }
                let backtrack_level = self.analyze(confl);
                self.vardata.cancel_trail_until(backtrack_level);

                if self.vardata.analyzer.learnt_clause.len() == 1 {
                    let p = self.vardata.analyzer.learnt_clause[0];
                    self.vardata.enqueue(p, CRef::UNDEF);
                } else {
                    let cref = self.db.alloc(&self.vardata.analyzer.learnt_clause, true);
                    self.watches
                        .watch(&self.vardata.analyzer.learnt_clause, cref);
                    self.vardata
                        .enqueue(self.vardata.analyzer.learnt_clause[0], cref);
                }
            } else {
                // No conflict
                loop {
                    if let Some(v) = self.vardata.order_heap.pop() {
                        if self.vardata.define(v) {
                            continue;
                        }

                        let lit = match self.vardata.polarity[v] {
                            LitBool::True => Lit::new(v.val(), true),
                            _ => Lit::new(v.val(), false),
                        };
                        self.vardata.trail.new_decision_level();
                        self.vardata.enqueue(lit, CRef::UNDEF);
                        break;
                    } else {
                        self.result = SatResult::Sat;
                        return SatResult::Sat;
                    }
                }
            }
        }
    }
    pub fn solve(&mut self) -> SatResult {
        if self.result != SatResult::Unknown {
            return self.result;
        }
        let mut result = SatResult::Unknown;
        while result == SatResult::Unknown {
            result = self.search();
        }

        if result == SatResult::Sat {
            self.models.resize(self.vardata.num_var(), LitBool::UnDef);
            for v in (0..self.vardata.num_var()).map(var::Var::from_idx) {
                self.models[v.val() as usize] = self.vardata.lbool(v);
            }
        }
        result
    }
}

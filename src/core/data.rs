use analyzer::Analyzer;

use crate::{
    clause::{alloc::CRef, db::ClauseDB},
    collections::{heap::Heap, idxvec::VarVec},
    types::{bool::LitBool, lit::Lit, var::Var},
};

use super::{analyzer, assign::AssignTrail};

/// VarData has basic information that is used for searching
pub struct VarData {
    /// assignments for each variable
    assigns: VarVec<LitBool>,
    /// decision level
    level: VarVec<u32>,
    /// CRef points a clause forces to assign a var.
    reason: VarVec<CRef>,
    /// a bunch of data is used to analyze conflicts.
    pub analyzer: Analyzer,
    pub trail: AssignTrail,
    /// polarity
    pub polarity: VarVec<LitBool>,
    /// the decision order
    pub order_heap: Heap,
}

impl VarData {
    pub fn new() -> VarData {
        VarData {
            assigns: VarVec::new(),
            level: VarVec::new(),
            reason: VarVec::new(),
            analyzer: Analyzer::new(),
            trail: AssignTrail::new(),
            polarity: VarVec::new(),
            order_heap: Heap::new(),
        }
    }
    pub fn num_var(&self) -> usize {
        self.assigns.len()
    }
    pub fn new_var(&mut self) {
        let v = Var(self.num_var() as u32);
        self.assigns.push(LitBool::default());
        self.level.push(0);
        self.reason.push(CRef::UNDEF);
        self.polarity.push(LitBool::True);
        self.analyzer.seen.push(false);

        self.order_heap.push(v);
    }

    fn redundant(&mut self, root: CRef, db: &mut ClauseDB) -> bool {
        // Check whether a literal can reach a decision variable or unit clause literal.
        // Self-subsume

        self.analyzer.ccmin_stack.clear();
        self.analyzer.ccmin_stack.push(root);

        let top = self.analyzer.ccmin_clear.len();
        let mut redundant = true;
        'redundant: while let Some(pos) = self.analyzer.ccmin_stack.pop() {
            let clause = db.get_mut(pos);

            for c in clause.iter().skip(1) {
                if !self.analyzer.seen[c.var()] && self.level(c.var()) > 0 {
                    // If a 'c' is decided by a level that is different from conflict literals.
                    // abstract_level(c) & abstract_levels == 0
                    let cref = self.reason(c.var());
                    if cref != CRef::UNDEF {
                        self.analyzer.seen[c.var()] = true;
                        self.analyzer.ccmin_stack.push(cref);
                        self.analyzer.ccmin_clear.push(*c);
                    } else {
                        redundant = false;
                        break 'redundant;
                    }
                }
            }
        }
        if !redundant {
            // A 'c' is a decision variable or unit clause literal.
            // which means a "lit" isn't redundant
            for lit in self.analyzer.ccmin_clear.iter().skip(top) {
                self.analyzer.seen[lit.var()] = false;
            }
            self.analyzer.ccmin_clear.truncate(top);
        }
        redundant
    }

    pub fn minimize_conflict_clause(&mut self, db: &mut ClauseDB) {
        debug_assert!(self.analyzer.ccmin_clear.is_empty());
        debug_assert!(self.analyzer.ccmin_stack.is_empty());

        let n = self.analyzer.learnt_clause.len();
        let mut new_size = 1;
        for i in 1..n {
            let lit = self.analyzer.learnt_clause[i];
            let redundant = {
                let cref = self.reason(lit.var());
                if cref != CRef::UNDEF {
                    self.redundant(cref, db)
                } else {
                    false
                }
            };
            if !redundant {
                self.analyzer.learnt_clause[new_size] = lit;
                new_size += 1;
            }
        }
        self.analyzer.learnt_clause.truncate(new_size);

        // clear
        for lit in self.analyzer.ccmin_clear.iter() {
            self.analyzer.seen[lit.var()] = false;
        }
        self.analyzer.ccmin_clear.clear();
        self.analyzer.ccmin_stack.clear();
    }

    pub fn cancel_trail_until(&mut self, backtrack_level: u32) {
        if self.trail.decision_level() <= backtrack_level {
            return;
        }
        let stack = &mut self.trail.stack;
        let sep = self.trail.stack_lim[backtrack_level as usize];
        for p in stack.iter().skip(sep).rev() {
            let v = p.var();
            self.order_heap.push(v);
            self.polarity[v] = p.true_lbool();
            self.assigns[v] = LitBool::UnDef;
            self.reason[v] = CRef::UNDEF;
            self.level[v] = 0;
        }
        self.trail.peek_head = sep;
        self.trail.stack.truncate(sep);
        self.trail.stack_lim.truncate(backtrack_level as usize);
    }

    fn assign(&mut self, var: Var, lb: LitBool, level: u32, reason: CRef) {
        debug_assert!(!self.define(var));
        debug_assert!(self.level(var) == 0);
        debug_assert!(self.reason[var] == CRef::UNDEF);
        self.assigns[var] = lb;
        self.level[var] = level;
        self.reason[var] = reason;
    }

    pub fn lbool(&self, var: Var) -> LitBool {
        self.assigns[var]
    }

    pub fn define(&self, var: Var) -> bool {
        self.assigns[var] != LitBool::UnDef
    }

    pub fn eval(&self, lit: Lit) -> LitBool {
        LitBool::from(self.assigns[lit.var()] as i8 ^ lit.neg() as i8)
    }

    pub fn level(&self, var: Var) -> u32 {
        self.level[var]
    }

    pub fn reason(&self, var: Var) -> CRef {
        self.reason[var]
    }

    pub fn enqueue(&mut self, lit: Lit, reason: CRef) {
        debug_assert!(self.eval(lit) == LitBool::UnDef);
        self.assign(
            lit.var(),
            lit.true_lbool(),
            self.trail.decision_level(),
            reason,
        );
        self.trail.push(lit);
    }
}

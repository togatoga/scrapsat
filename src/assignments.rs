use std::collections::BinaryHeap;

use crate::index_vec::Idx;
use crate::lit::{Lit, LitBool};
use crate::{clause::ClauseRef, Var};

pub struct VarData {
    reason: Option<ClauseRef>,
    level: usize, //the decision level
}

impl VarData {
    pub fn new(reason: Option<ClauseRef>, level: usize) -> VarData {
        VarData { reason, level }
    }
}

#[derive(Default)]
pub struct Assignment {
    pub assigns: Vec<LitBool>,   //Var to LitBool: The current assignments.
    decision: Vec<bool>, // Declares if a variable is eligible for selection in the decision heuristic.
    var_data: Vec<VarData>, //Var to VarData: Stores reason and level for each variable
    pub trail: Vec<Lit>, //Assignment stack; stores all assignments made in the order they were made.
    trail_lim: Vec<usize>, //Separator indices for different decision levels in 'trail'.
    pub head: usize, // Head of queue (as index into the trail -- no more explicit propagation queue in MiniSat).
    order_heap: BinaryHeap<Var>, // A priority queue of variables ordered with respect to the variable activity.
    // Variables for clause learning
    pub seen: Vec<bool>,
}

impl Assignment {
    pub fn new() -> Assignment {
        Assignment {
            assigns: Vec::new(),
            decision: Vec::new(),
            var_data: Vec::new(),
            trail: Vec::new(),
            trail_lim: Vec::new(),
            head: 0,
            order_heap: BinaryHeap::new(),
            seen: Vec::new(),
        }
    }
    pub fn assign_true(&mut self, p: Lit, reason: Option<ClauseRef>) {
        debug_assert!(self.is_assigned_undef(p));
        self.assigns[p.var().idx()] = if p.is_pos() {
            LitBool::True
        } else {
            LitBool::False
        };
        self.var_data[p.var().idx()] = VarData {
            reason,
            level: self.current_decision_level(),
        };
        self.trail.push(p);
    }

    pub fn pop_front_trail(&mut self) -> Option<Lit> {
        if self.head < self.trail.len() {
            let p = self.trail[self.head];
            self.head += 1;
            Some(p)
        } else {
            None
        }
    }

    pub fn is_assigned_true(&self, p: Lit) -> bool {
        p.is_true(self.assigns[p.var().idx()])
    }

    pub fn is_assigned_false(&self, p: Lit) -> bool {
        p.is_false(self.assigns[p.var().idx()])
    }

    pub fn is_assigned_undef(&self, p: Lit) -> bool {
        self.assigns[p.var().idx()] == LitBool::Undef
    }

    pub fn current_decision_level(&self) -> usize {
        self.trail_lim.len()
    }

    pub fn decision_level(&self, p: Lit) -> usize {
        self.var_data[p.var().idx()].level
    }
    pub fn reason(&self, p: Lit) -> Option<ClauseRef> {
        self.var_data[p.var().idx()].reason
    }

    pub fn new_decision_level(&mut self) {
        self.trail_lim.push(self.trail.len());
    }

    pub fn n_var(&self) -> usize {
        self.assigns.len()
    }

    pub fn seen(&self, p: Lit) -> bool {
        self.seen[p.var().idx()]
    }
    pub fn check(&mut self, p: Lit) {
        assert!(!self.seen[p.var().idx()]);
        self.seen[p.var().idx()] = false;
    }
    pub fn uncheck(&mut self, p: Lit) {
        assert!(self.seen(p));
        self.seen[p.var().idx()] = true;
    }

    pub fn pick_bracnh_lit(&mut self) -> Option<Lit> {
        while let Some(p) = self.order_heap.pop() {
            if self.assigns[p.idx()] != LitBool::Undef || !self.decision[p.idx()] {
                continue;
            }
            return Some(Lit::new(p, false));
        }
        None
    }

    // Revert to the state at given level (keeping all assignment at 'level' but not beyond).
    pub fn cancel_until(&mut self, level: usize) {
        if self.current_decision_level() > level {
            for c in (self.trail_lim[level]..self.trail.len()).rev() {
                let x = self.trail[c].var();
                assert_ne!(self.assigns[x.idx()], LitBool::Undef);
                self.assigns[x.idx()] = LitBool::Undef;
                self.order_heap.push(x);
            }
            self.head = self.trail_lim[level];
            self.trail
                .truncate(self.trail.len() - self.trail_lim[level]);
            self.trail_lim.truncate(self.trail_lim.len() - level);
        }
    }

    pub fn new_var(&mut self) {
        debug_assert_eq!(self.current_decision_level(), 0);
        let v = Var(self.n_var() as i32);
        self.trail.reserve(self.n_var() + 1);
        //push new a var
        self.assigns.push(LitBool::Undef);
        self.var_data.push(VarData::new(None, 0));
        self.seen.push(false);
        self.decision.push(true);
        self.order_heap.push(v);
    }
}

use crate::clause::ClauseRef;
use crate::index_vec::Idx;
use crate::lit::{Lit, LitBool};

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
    assigns: Vec<LitBool>,  //Var to LitBool: The current assignments.
    var_data: Vec<VarData>, //Var to VarData: Stores reason and level for each variable
    trail: Vec<Lit>, //Assignment stack; stores all assignments made in the order they were made.
    trail_lim: Vec<usize>, //Separator indices for different decision levels in 'trail'.
    head: usize, // Head of queue (as index into the trail -- no more explicit propagation queue in MiniSat).
}

impl Assignment {
    pub fn new() -> Assignment {
        Assignment {
            assigns: Vec::new(),
            var_data: Vec::new(),
            trail: Vec::new(),
            trail_lim: Vec::new(),
            head: 0,
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

    pub fn front_trail(&mut self) -> Option<Lit> {
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
    pub fn new_decision_level(&mut self) {
        self.trail_lim.push(self.trail.len());
    }

    pub fn n_var(&self) -> usize {
        self.assigns.len()
    }

    pub fn new_var(&mut self) {
        debug_assert_eq!(self.current_decision_level(), 0);
        self.trail.reserve(self.n_var() + 1);
        //push new a var
        self.assigns.push(LitBool::Undef);
        self.var_data.push(VarData::new(None, 0));
    }
}

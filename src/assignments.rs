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

pub struct Assignment {
    assigns: Vec<LitBool>,  //The current assignments.
    var_data: Vec<VarData>, //Stores reason and level for each variable
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

    pub fn is_assigned_true(&self, p: Lit) -> bool {
        p.is_true(self.assigns[p.var().idx()])
    }

    pub fn is_assigned_false(&self, p: Lit) -> bool {
        p.is_false(self.assigns[p.var().idx()])
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

use crate::types::lit::Lit;

#[derive(Debug, Default)]
pub struct AssignTrail {
    /// Stores all assignments made in the order they made.
    stack: Vec<Lit>,
    /// Separator indices for different decision levels in `stack`
    /// e.g. For `stack`,
    /// 0..stack_lim[0] is all assignments at 0 level
    /// stack_lim[0]..stack_lim[1] is all assignments  at 1 level
    stack_lim: Vec<usize>,
    /// Head of `stack`
    peek_head: usize,
}

impl AssignTrail {
    #[allow(dead_code)]
    pub fn new() -> AssignTrail {
        AssignTrail {
            stack: Vec::new(),
            stack_lim: Vec::new(),
            peek_head: 0,
        }
    }

    /// Increase the level of decision.
    pub fn new_decision_level(&mut self) {
        self.stack_lim.push(self.stack.len());
    }
    /// Returns the current level of decision.
    pub fn decision_level(&self) -> u32 {
        self.stack_lim.len() as u32
    }

    /// Returns a boolean whether `peek_head` is within `stack`.
    pub fn peekable(&self) -> bool {
        self.peek_head < self.stack.len()
    }

    /// Returns a `lit` of `peek_head` `stack` .
    pub fn peek(&self) -> Lit {
        debug_assert!(self.peekable());
        self.stack[self.peek_head]
    }

    /// Pop the `peek-head` `stack`
    pub fn pop(&mut self) -> Option<Lit> {
        let res = if self.peekable() {
            Some(self.peek())
        } else {
            None
        };
        self.advance();
        res
    }

    /// Advance `peek_head`
    pub fn advance(&mut self) {
        self.peek_head += 1;
    }

    /// Push a new Lit to `stack`.
    pub fn push(&mut self, x: Lit) {
        self.stack.push(x);
    }

    /// Returns the number of assignment
    pub fn num_assign(&self) -> usize {
        self.stack.len()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;
    #[test]
    fn test_assign_trail() {
        let mut trail = AssignTrail::new();
        let mut lits: VecDeque<_> =
            vec![Lit::new(0, true), Lit::new(1, false), Lit::new(2, true)].into();
        for &lit in lits.iter() {
            trail.push(lit);
        }

        assert_eq!(trail.num_assign(), 3);
        assert_eq!(trail.decision_level(), 0);
        trail.new_decision_level();
        assert_eq!(trail.decision_level(), 1);
        while let Some(x) = trail.pop() {
            eprintln!("{:?}", x);
            assert!(lits.pop_front() == Some(x));
        }
    }
}

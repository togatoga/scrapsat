use crate::index_vec::Idx;
use crate::Var;
use std::ops::Not;

//This value is used for literal assignments
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
pub enum LitBool {
    False,
    True,
    Undef,
}

//Lit represents a positive and negative variable like x1 and ¬x1
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Default, Copy)]
pub struct Lit {
    x: i32,
}

impl Lit {
    pub fn new(x: Var, neg: bool) -> Lit {
        debug_assert!(x.0 >= 0);
        if neg {
            Lit { x: 2 * x.0 + 1 }
        } else {
            Lit { x: 2 * x.0 }
        }
    }

    //pos return a boolean whether lit is positive or not.
    pub fn is_pos(self) -> bool {
        self.x & 1 == 0
    }
    //neg returns a boolean whether lit is negative or not.
    pub fn is_neg(self) -> bool {
        self.x & 1 != 0
    }

    pub fn is_false(self, val: LitBool) -> bool {
        match val {
            LitBool::False => !self.is_neg(),
            LitBool::True => self.is_neg(),
            LitBool::Undef => false,
        }
    }

    pub fn is_true(self, val: LitBool) -> bool {
        match val {
            LitBool::False => self.is_neg(),
            LitBool::True => !self.is_neg(),
            LitBool::Undef => false,
        }
    }

    pub fn var(self) -> Var {
        Var(self.x >> 1)
    }
}

impl Idx for Lit {
    fn idx(&self) -> usize {
        self.x as usize
    }
}

//e.g.
// x1 -> ¬x1
// ¬x1 -> x1
impl Not for Lit {
    type Output = Self;
    fn not(self) -> Self::Output {
        Lit { x: self.x ^ 1 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lit() {
        //x1
        let lit = Lit::new(Var(0), false);
        assert_eq!(lit.x, 0);
        assert_eq!(lit.is_neg(), false);
        assert_eq!(lit.is_pos(), true);
        //¬x1
        let lit = Lit::new(Var(0), true);
        assert_eq!(lit.x, 1);
        assert_eq!(lit.is_neg(), true);
        assert_eq!(lit.is_pos(), false);
        //not
        assert_eq!(!lit, Lit::new(Var(0), false));

        let lit = Lit::new(Var(1), true);
        assert_eq!(lit.x, 3);

        let lit = Lit::new(Var(0), false);
        assert_eq!(lit.is_true(LitBool::True), true);
        assert_eq!(lit.is_true(LitBool::False), false);

        let lit = Lit::new(Var(0), true);
        assert_eq!(lit.is_true(LitBool::True), false);
        assert_eq!(lit.is_true(LitBool::False), true);
        assert_eq!(lit.is_true(LitBool::Undef), false);

        assert_eq!(lit.is_false(LitBool::True), true);
        assert_eq!(lit.is_false(LitBool::False), false);
        assert_eq!(lit.is_false(LitBool::Undef), false);

        let lit = Lit::new(Var(0), false);
        assert_eq!(lit.is_true(LitBool::True), true);
        assert_eq!(lit.is_true(LitBool::False), false);
        assert_eq!(lit.is_true(LitBool::Undef), false);

        assert_eq!(lit.is_false(LitBool::True), false);
        assert_eq!(lit.is_false(LitBool::False), true);
        assert_eq!(lit.is_false(LitBool::Undef), false);
    }
}

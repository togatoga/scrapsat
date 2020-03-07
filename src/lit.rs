use crate::Var;
use std::ops::Not;

//This value is used for literal assignments
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub enum LitBool {
    False,
    True,
    Undef,
}

//Lit represents a positive and negative variable like x1 and ¬x1
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub struct Lit {
    x: i32,
}

impl Lit {
    pub fn new(x: i32, neg: bool) -> Lit {
        debug_assert!(x >= 0);
        if neg {
            Lit { x: 2 * x + 1 }
        } else {
            Lit { x: 2 * x }
        }
    }

    //pos return a boolean whether lit is positive or not.
    pub fn pos(&self) -> bool {
        self.x & 1 == 0
    }
    //neg returns a boolean whether lit is negative or not.
    pub fn neg(&self) -> bool {
        self.x & 1 != 0
    }

    pub fn is_false(&self, val: LitBool) -> bool {
        match val {
            LitBool::False => !self.neg(),
            LitBool::True => self.neg(),
            LitBool::Undef => false,
        }
    }

    pub fn is_true(&self, val: LitBool) -> bool {
        match val {
            LitBool::False => self.neg(),
            LitBool::True => !self.neg(),
            LitBool::Undef => false,
        }
    }

    pub fn var(&self) -> Var {
        self.x >> 1
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
        let lit = Lit::new(0, false);
        assert_eq!(lit.x, 0);
        assert_eq!(lit.neg(), false);
        //¬x1
        let lit = Lit::new(0, true);
        assert_eq!(lit.x, 1);
        assert_eq!(lit.neg(), true);
        //not
        assert_eq!(!lit, Lit::new(0, false));

        let lit = Lit::new(1, true);
        assert_eq!(lit.x, 3);

        let lit = Lit::new(0, false);
        assert_eq!(lit.is_true(LitBool::True), true);
        assert_eq!(lit.is_true(LitBool::False), false);

        let lit = Lit::new(0, true);
        assert_eq!(lit.is_true(LitBool::True), false);
        assert_eq!(lit.is_true(LitBool::False), true);
        assert_eq!(lit.is_true(LitBool::Undef), false);

        assert_eq!(lit.is_false(LitBool::True), true);
        assert_eq!(lit.is_false(LitBool::False), false);
        assert_eq!(lit.is_false(LitBool::Undef), false);

        let lit = Lit::new(0, false);
        assert_eq!(lit.is_true(LitBool::True), true);
        assert_eq!(lit.is_true(LitBool::False), false);
        assert_eq!(lit.is_true(LitBool::Undef), false);

        assert_eq!(lit.is_false(LitBool::True), false);
        assert_eq!(lit.is_false(LitBool::False), true);
        assert_eq!(lit.is_false(LitBool::Undef), false);
    }
}

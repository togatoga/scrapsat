use super::{bool::LitBool, var::Var};

/// A Literal(`Lit`) is a positive or negative boolean variable(`Var`).
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Lit(u32);
impl Lit {
    /// A `UNDEF` is a default lit.
    pub const UNDEF: Lit = Lit(std::u32::MAX);
    pub fn new(var: u32, positive: bool) -> Lit {
        Lit(if positive { var << 1 } else { (var << 1) + 1 })
    }
    #[inline]
    pub fn var(self) -> Var {
        Var(self.0 >> 1)
    }

    /// A `lit` is positive(x1)
    #[inline]
    pub fn pos(&self) -> bool {
        self.0 & 1 == 0
    }

    /// A `lit` is negative(!x1)
    #[inline]
    pub fn neg(&self) -> bool {
        self.0 & 1 != 0
    }

    #[inline]
    pub fn undefine(&self) -> bool {
        *self == Lit::UNDEF
    }
    #[inline]
    pub fn define(&self) -> bool {
        *self != Lit::UNDEF
    }

    #[inline]
    pub fn val(&self) -> u32 {
        self.0
    }

    #[inline]
    pub fn true_lbool(&self) -> LitBool {
        if self.pos() {
            LitBool::True
        } else {
            LitBool::False
        }
    }
}

impl Default for Lit {
    fn default() -> Self {
        Lit::UNDEF
    }
}

impl std::fmt::Debug for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if *self == Lit::UNDEF {
            write!(f, "Lit::UNDEF")
        } else {
            write!(f, "Lit({}, {})", self.var().0, self.pos())
        }
    }
}

impl From<i32> for Lit {
    #[inline]
    fn from(x: i32) -> Self {
        debug_assert!(x != 0, "0 can not be positive or negative");
        let d = x.abs() as u32 - 1;
        if x > 0 {
            Lit(d << 1)
        } else {
            Lit((d << 1) + 1)
        }
    }
}

impl From<Var> for Lit {
    fn from(var: Var) -> Self {
        Lit(var.0)
    }
}

impl std::ops::Not for Lit {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Lit(self.0 ^ 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lit() {
        let x = Lit::default();
        assert_eq!(x, Lit::UNDEF, "x is UNDEFINE");
        assert!(x.undefine());

        let x = Lit::new(0, true);
        assert!(x.pos());
        assert_eq!(x.var(), Var(0));
        assert_eq!(x.val(), 0);
        assert_eq!((!x).val(), 1);
    }
}

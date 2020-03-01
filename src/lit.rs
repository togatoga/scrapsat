use crate::Var;
//Lit represents a positive and negative variable like x1 and ¬x1
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Lit {
    x: i32,
}

impl Lit {
    pub fn new(x: i32, neg: bool) -> Lit {
        assert!(x >= 0);
        if neg {
            Lit { x: 2 * x + 1 }
        } else {
            Lit { x: 2 * x }
        }
    }
    //neg returns a boolean whether lit is negative or not.
    pub fn neg(&self) -> bool {
        if self.x % 2 != 0 {
            true
        } else {
            false
        }
    }

    pub fn var(&self) -> Var {
        self.x >> 1
    }

    //flip_sign return a Lit whose sign is flipped
    //e.g.
    // x1 -> ¬x1
    // ¬x1 -> x1
    pub fn flip_sign(&self) -> Lit {
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
        //flip
        assert_eq!(lit.flip_sign(), Lit::new(0, false));

        let lit = Lit::new(1, true);
        assert_eq!(lit.x, 3);
    }
}

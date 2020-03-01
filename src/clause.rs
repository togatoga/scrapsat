use crate::lit::Lit;
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Clause {
    lits: Vec<Lit>,
}

impl Clause {
    pub fn new(lits: &[Lit]) -> Clause {
        Clause {
            lits: lits.to_vec(),
        }
    }
}

type ClauseRef = usize;
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct ClauseAllocator {
    ra: Vec<Clause>, //NOTE It's better to replace this with raw pointer region allocator to improve performance
}

impl ClauseAllocator {
    pub fn with_capacity(capacity: usize) -> ClauseAllocator {
        ClauseAllocator {
            ra: Vec::with_capacity(capacity),
        }
    }
    pub fn alloc(&mut self, lits: &[Lit]) -> ClauseRef {
        let cref = self.ra.len();
        self.ra.push(Clause::new(lits));
        return cref;
    }

    //lazy_free free a specified region but only mark a clause deleted. not to free actual region
    pub fn lazy_free(&mut self, cref: ClauseRef) {
        let clause = self.clause_mut(cref);
        //TODO
        //Delete it
    }

    pub fn clause(&self, cref: ClauseRef) -> &Clause {
        &self.ra[cref]
    }
    pub fn clause_mut(&mut self, cref: ClauseRef) -> &mut Clause {
        &mut self.ra[cref]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_clause_allocator() {
        let mut cla = ClauseAllocator::with_capacity(1024 * 1024);
        let lits = vec![Lit::new(0, false), Lit::new(1, false), Lit::new(3, true)];
        let cref = cla.alloc(&lits);
        assert_eq!(cref, 0);
        let clause = cla.clause(cref);
        assert_eq!(clause.lits, lits);
        let clause = cla.clause_mut(cref);
        clause.lits[0] = Lit::new(1, true);
        assert_eq!(
            clause.lits,
            vec![Lit::new(1, true), Lit::new(1, false), Lit::new(3, true)]
        );
    }
}

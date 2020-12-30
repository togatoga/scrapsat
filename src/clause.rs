use crate::lit::Lit;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Default)]
pub struct ClauseHeader {
    bit: u32,
}

impl ClauseHeader {
    const BIT_DELETED: u32 = 0x01;
    const BIT_TOUCHED: u32 = 0x02;
    const BIT_LEARNT: u32 = 0x03;

    pub fn new() -> ClauseHeader {
        ClauseHeader { bit: 0 }
    }
    #[allow(dead_code)]
    fn mark_learnt(&mut self) {
        self.bit |= ClauseHeader::BIT_LEARNT;
    }
    #[allow(dead_code)]
    fn is_learnt(&mut self) -> bool {
        self.bit & ClauseHeader::BIT_LEARNT != 0
    }
    //NOTE
    //TOUCHED value might be only used in simp
    #[allow(dead_code)]
    pub fn mark_touched(&mut self) {
        self.bit |= ClauseHeader::BIT_TOUCHED;
    }
    #[allow(dead_code)]
    pub fn is_touched(&self) -> bool {
        self.bit & ClauseHeader::BIT_TOUCHED != 0
    }

    pub fn mark_deleted(&mut self) {
        self.bit |= ClauseHeader::BIT_DELETED;
    }
    pub fn is_deleted(&self) -> bool {
        self.bit & ClauseHeader::BIT_DELETED != 0
    }
    #[allow(dead_code)]
    pub fn clear_mark(&mut self) {
        self.bit = 0;
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Clause {
    pub lits: Vec<Lit>,
    pub header: ClauseHeader,
}

impl Clause {
    pub fn new(lits: &[Lit]) -> Clause {
        Clause {
            lits: lits.to_vec(),
            header: ClauseHeader::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.lits.is_empty()
    }
    pub fn len(&self) -> usize {
        self.lits.len()
    }
}

impl std::ops::Index<usize> for Clause {
    type Output = Lit;
    fn index(&self, idx: usize) -> &Lit {
        &self.lits[idx]
    }
}
impl std::ops::IndexMut<usize> for Clause {
    fn index_mut(&mut self, idx: usize) -> &mut Lit {
        &mut self.lits[idx]
    }
}

pub type ClauseRef = usize;
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct ClauseAllocator {
    ra: Vec<Clause>, //NOTE It's better to replace this with raw pointer region allocator to improve the performance
}

impl ClauseAllocator {
    pub fn new() -> ClauseAllocator {
        ClauseAllocator { ra: Vec::new() }
    }
    pub fn with_capacity(capacity: usize) -> ClauseAllocator {
        ClauseAllocator {
            ra: Vec::with_capacity(capacity),
        }
    }
    pub fn alloc(&mut self, lits: &[Lit]) -> ClauseRef {
        let cref = self.ra.len();
        self.ra.push(Clause::new(lits));
        cref
    }

    //lazy_free free a specified region but only mark a clause deleted. not to free actual region
    pub fn lazy_free(&mut self, cref: ClauseRef) {
        let clause = self.clause_mut(cref);
        debug_assert!(clause.header.is_deleted());
        clause.header.mark_deleted();
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
    use crate::Var;

    #[test]
    fn test_clause_allocator() {
        let mut cla = ClauseAllocator::with_capacity(1024 * 1024);
        let lits = vec![
            Lit::new(Var(0), false),
            Lit::new(Var(1), false),
            Lit::new(Var(3), true),
        ];
        let cref = cla.alloc(&lits);
        assert_eq!(cref, 0);
        let clause = cla.clause(cref);
        assert_eq!(clause.lits, lits);
        let clause = cla.clause_mut(cref);
        clause.lits[0] = Lit::new(Var(1), true);
        assert_eq!(
            clause.lits,
            vec![
                Lit::new(Var(1), true),
                Lit::new(Var(1), false),
                Lit::new(Var(3), true)
            ]
        );
    }

    #[test]
    fn test_clause_header() {
        let mut header = ClauseHeader::new();
        assert!(!header.is_deleted());
        header.mark_learnt();
        assert!(header.is_learnt());
        header.mark_deleted();
        assert!(header.is_deleted());
    }
}

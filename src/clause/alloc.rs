use std::marker::PhantomData;

use crate::{
    collections::alloc::{Id, RegionAllocator},
    types::lit::Lit,
};

use super::{
    word::{ClauseWord, Flags},
    Clause,
};

pub type CRef = Id<ClauseWord>;

pub struct ClauseAllocator {
    ra: RegionAllocator<ClauseWord>,
    wasted: u32,
}

impl ClauseAllocator {
    pub fn new() -> ClauseAllocator {
        ClauseAllocator {
            ra: RegionAllocator::new(),
            wasted: 0,
        }
    }

    /// [flags, len, lit0, lit1, lit2]
    /// [flags, len, lit0, lit1, lit2, extra]
    pub fn alloc(&mut self, lits: &[Lit], learnt: bool) -> CRef {
        let flags = if learnt { Flags::LEARNT } else { Flags::NONE };
        //flags
        let src = self.ra.alloc(ClauseWord::from(flags));
        //len
        self.ra.alloc(ClauseWord::from(lits.len()));
        for &lit in lits.iter() {
            self.ra.alloc(ClauseWord::from(lit));
        }
        //activity
        if learnt {
            self.ra.alloc(ClauseWord::from(0.0f32));
        }
        src
    }

    fn region_len(&self, cref: CRef) -> u32 {
        let flags = unsafe { self.ra.get(cref).flags };
        let len_clause = unsafe { self.ra.get(cref + 1).len };
        // flags + len + lit0 + lit1 + extra
        if flags.contains(Flags::LEARNT) {
            1 + 1 + len_clause + 1
        } else {
            1 + 1 + len_clause
        }
    }

    pub fn free(&mut self, cref: CRef) {
        let mut flags = unsafe { self.ra.get(cref).flags };
        debug_assert!(!flags.contains(Flags::DELTED));
        flags.insert(Flags::DELTED);
        self.wasted += self.region_len(cref);
    }

    pub fn get_mut(&mut self, cref: CRef) -> Clause {
        let flags = unsafe { self.ra.get(cref).flags };
        let len_clause = unsafe { self.ra.get(cref + 1).len };
        let len = if flags.contains(Flags::LEARNT) {
            1 + 1 + len_clause + 1
        } else {
            1 + 1 + len_clause
        };
        let slice = self.ra.subslice_mut(cref, len as usize);
        //let flags = unsafe { &mut slice.get_unchecked_mut(0).flags };
        let (flag_slice, slice) = slice.split_at_mut(1);
        let (len_slice, slice) = slice.split_at_mut(1);
        let (data_slice, extra_slice) = slice.split_at_mut(len_clause as usize);

        //eprintln!("{:?}", unsafe {len_slice[0].len});
        Clause {
            flags: unsafe { &mut flag_slice[0].flags },
            data: data_slice,
            extra: extra_slice.first_mut(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::lit::Lit;

    use super::ClauseAllocator;

    #[test]
    fn test_clause_allocator() {
        let mut ca = ClauseAllocator::new();

        {
            // no learnt
            let n = 1024;
            let lits: Vec<Lit> = (0..n).map(|i| Lit::new(i, i % 2 == 0)).collect();

            let cref = ca.alloc(&lits, false);
            let mut clause = ca.get_mut(cref);
            assert!(!clause.learnt());
            for (i, lit) in clause.iter().enumerate() {
                assert_eq!(Lit::new(i as u32, i % 2 == 0), *lit);
            }
            clause[0] = Lit::new(2048, true);

            let c = ca.get_mut(cref);
            assert_eq!(c[0], Lit::new(2048, true));
        }

        {
            // learnt
            let n = 10;
            let lits: Vec<Lit> = (0..n).map(|i| Lit::new(i, true)).collect();

            let cref = ca.alloc(&lits, true);
            let clause = ca.get_mut(cref);
            assert!(clause.learnt());
            for (i, lit) in clause.iter().enumerate() {
                assert_eq!(Lit::new(i as u32, true), *lit);
            }
            assert_eq!(clause.activity(), 0.0);
        }
    }
}

use bitflags::bitflags;

use crate::types::lit::Lit;

use super::alloc::CRef;

/// `ClauseWord` represents the unit of data that is stored in the allocator.
#[derive(Clone, Copy)]
pub union ClauseWord {
    pub flags: Flags,
    /// length of a clause
    pub len: u32,
    pub lit: Lit,
    /// only for learnt clause
    pub activity: f32,
    pub relocation: CRef,
}

impl From<Flags> for ClauseWord {
    fn from(flags: Flags) -> Self {
        ClauseWord { flags }
    }
}

impl From<usize> for ClauseWord {
    fn from(len: usize) -> Self {
        ClauseWord { len: len as u32 }
    }
}

impl From<Lit> for ClauseWord {
    fn from(lit: Lit) -> Self {
        ClauseWord { lit }
    }
}

impl From<f32> for ClauseWord {
    fn from(activity: f32) -> Self {
        ClauseWord { activity }
    }
}

bitflags! {
    /// Flags represents binary clause data
    pub struct Flags: u32 {
        const NONE = 0b00000000;
        /// A clause is deleted
        const DELTED = 0b00000001;
        /// A clause is learnt
        const LEARNT = 0b00000010;
        /// A clause is relocated
        const RELOCATED = 0b00000100;
    }
}


use crate::global_state::FileRef;

// TODO(jez) Sorbet fits begin / end into 24 bits, so that a Loc fits in a single u64
// Use this crate[1] to pack a Loc into a single u64
// [1] https://docs.rs/bitfield/0.13.2/bitfield/

#[derive(Debug, Clone, Copy)]
pub struct Loc {
    pub begin: usize,
    pub end: usize,
    pub file: FileRef,
}

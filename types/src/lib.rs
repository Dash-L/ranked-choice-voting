use serde::{Deserialize, Deserializer, Serialize};
use std::num::{NonZeroU8, NonZeroUsize};

pub use rmp_serde;
pub use serde;

pub type Candidate = NonZeroUsize;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Ballot {
    #[serde(skip)]
    pub selected: usize,
    pub votes: Vec<Option<Candidate>>,
}

type CandidateBetter = NonZeroU8;

#[derive(Serialize, Clone)]
pub struct BallotBetter<'mmap> {
    #[serde(skip)]
    pub selected: u8,
    pub votes: &'mmap [Option<CandidateBetter>],
}

impl<'mmap> BallotBetter<'mmap> {
    pub fn next(mmap: &mut &'mmap [u8]) -> Option<Self> {
        if mmap.len() == 0 {
            return None;
        }

        let next_two_bytes = &mmap[..2];
        debug_assert!(
            next_two_bytes == [0x91, 0x95],
            "expected msgpack array start bytes [0x91, 0x95], found: {:?}",
            next_two_bytes
        );

        // Read 5 bytes for the votes
        let current_votes = &mmap[2..7];

        // Advance the reader by 7 bytes
        *mmap = &mmap[7..];

        Some(BallotBetter {
            selected: 0,
            // This is probably safe because the memory layout is the same:
            // - `None` is represented as `0u8`
            // - `Some(x)` is represented as `x` as u8
            votes: unsafe { std::mem::transmute(current_votes) },
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct CandidateEntry {
    id: usize,
    name: String,
}

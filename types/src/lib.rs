use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Clone)]
pub struct BallotBetter {
    #[serde(skip)]
    pub selected: u8,
    pub votes: Vec<Option<CandidateBetter>>,
}

#[derive(Serialize, Deserialize)]
pub struct CandidateEntry {
    id: usize,
    name: String,
}

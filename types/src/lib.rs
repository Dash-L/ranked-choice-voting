use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;

pub use rmp_serde;
pub use serde;

pub type Candidate = NonZeroUsize;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Ballot {
    #[serde(skip)]
    pub selected: usize,
    pub votes: Vec<Option<Candidate>>,
}

#[derive(Serialize, Deserialize)]
pub struct CandidateEntry {
    id: usize,
    name: String,
}

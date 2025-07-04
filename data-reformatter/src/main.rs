use std::{collections::HashMap, env, num::NonZeroU8};

use types::{
    Ballot, BallotBetter,
    serde::{Deserialize, Serialize},
};

fn main() {
    let vote_data_path = &std::fs::canonicalize(format!(
        "{}/../vote_data.mpack",
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set")
    ))
    .expect("Failed to canonicalize vote data path");

    let data = std::fs::read(vote_data_path).expect("Failed to read vote data file");

    let mut deserializer = rmp_serde::Deserializer::new(&data[..]);

    let mut better_ballots: Vec<BallotBetter> = Vec::new();

    let mut new_candidate_ids = HashMap::<usize, NonZeroU8>::new();

    while let Ok(bad_ballot) = Ballot::deserialize(&mut deserializer) {
        let better_ballot = BallotBetter {
            // This is ignored it really doesn't matter
            selected: 0,
            votes: bad_ballot
                .votes
                .into_iter()
                .map(|v| {
                    v.map(|c| {
                        let len = new_candidate_ids.len();
                        // Convert the candidate ID to a new ID
                        *new_candidate_ids.entry(c.get()).or_insert_with(|| {
                            // Generate a new ID starting from 1
                            NonZeroU8::new((len + 1) as u8).unwrap()
                        })
                    })
                })
                .collect(),
        };

        better_ballots.push(better_ballot);
    }

    // Serialize the better ballots to a new file
    let out_path = "better_vote_data.mpack";
    let mut out_file = std::fs::File::create(out_path).expect("Could not open output file");
    let mut serializer = rmp_serde::Serializer::new(&mut out_file);
    for ballot in better_ballots {
        ballot
            .serialize(&mut serializer)
            .expect("Failed to serialize ballot");
    }
}

use std::{collections::HashMap, env, num::NonZeroU8};

use types::{
    Ballot,
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

    let mut new_candidate_ids = HashMap::<usize, NonZeroU8>::new();

    let out_path = "better_vote_data.mpack";
    let mut out_file = std::fs::File::create(out_path).expect("Could not open output file");
    let mut serializer = rmp_serde::Serializer::new(&mut out_file);

    while let Ok(bad_ballot) = Ballot::deserialize(&mut deserializer) {
        let votes = bad_ballot
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
            .map(|v| {
                v.map(|c| c.get()).unwrap_or(0) // Use 0 for nil votes
            })
            .collect::<Vec<_>>();

        #[derive(serde::Serialize)]
        pub struct BallotBetterWithNullPointerOptimization {
            pub votes: Vec<u8>,
        }

        let better_ballot = BallotBetterWithNullPointerOptimization { votes };

        better_ballot
            .serialize(&mut serializer)
            .expect("Failed to serialize ballot");
    }
}

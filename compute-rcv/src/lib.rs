#![feature(file_buffered)]

use fnv::FnvHashMap;
use std::num::NonZeroUsize;
use std::{env, fs::File};

use types::{Ballot, rmp_serde::Deserializer, serde::Deserialize};

pub fn count_rcv() {
    let vote_data_path = &std::fs::canonicalize(format!(
        "{}/../vote_data.mpack",
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set")
    ))
    .expect("Failed to canonicalize vote data path");

    // let now = std::time::Instant::now();
    let vote_data_file = File::open_buffered(vote_data_path).expect(&format!(
        "failed to open vote data file: {:?}",
        vote_data_path
    ));
    let mut deserializer = Deserializer::new(vote_data_file);

    let mut ballots = Vec::new();
    let mut candidate_votes = FnvHashMap::default();

    while let Ok(mut b) = Ballot::deserialize(&mut deserializer) {
        let first_vote = b
            .votes
            .iter()
            .enumerate()
            .find(|(_, v)| v.is_some())
            .map(|(i, v)| (i, v.clone()));

        // let now = std::time::Instant::now();
        if let Some((first_vote_index, Some(first_vote))) = first_vote {
            b.selected = first_vote_index;

            ballots.push(b);
            let ballot_idx = ballots.len() - 1;

            candidate_votes
                .entry(first_vote)
                .or_insert_with(|| vec![])
                .push(ballot_idx);
        }
        // println!("[timing] sorting of ballot took: {:?}", now.elapsed());
    }

    // println!("[timing] deserialization took: {:?}", now.elapsed());

    // let now = std::time::Instant::now();
    let mut items = candidate_votes.iter().collect::<Vec<_>>();
    items.sort_by_key(|(_k, v)| v.len());

    for (k, v) in items {
        println!("Candidate {} has {} votes", k, v.len());
    }

    println!("Total valid votes: {}", ballots.len());

    // println!("[timing] sorting took: {:?}", now.elapsed());

    let mut total_valid_ballots = ballots.len();

    loop {
        let mut best_id: NonZeroUsize = NonZeroUsize::new(1).unwrap();
        let mut best_count: usize = 0;
        let mut worst_id: NonZeroUsize = NonZeroUsize::new(1).unwrap();
        let mut worst_count: usize = usize::MAX;

        candidate_votes.iter().for_each(|(id, votes)| {
            if votes.len() > best_count {
                best_id = *id;
                best_count = votes.len();
            }
            if votes.len() < worst_count {
                worst_id = *id;
                worst_count = votes.len();
            }
        });

        if best_count as f64 / total_valid_ballots as f64 > 0.5 {
            println!(
                "The winner is {} with {} votes ({:.3}%)",
                best_id,
                best_count,
                best_count as f64 / total_valid_ballots as f64 * 100.
            );
            break;
        }

        println!(
            "The current leader is {} with {} votes ({:.3}%)",
            best_id,
            best_count,
            best_count as f64 / total_valid_ballots as f64 * 100.
        );

        let worst_votes = candidate_votes.remove(&worst_id).unwrap();
        'reassign: for b in worst_votes {
            let b_mut = &mut ballots[b];

            for i in (b_mut.selected + 1)..b_mut.votes.len() {
                if let Some(next_candidate) = b_mut.votes[i]
                    && candidate_votes.contains_key(&next_candidate)
                {
                    b_mut.selected = i;
                    candidate_votes
                        .get_mut(&next_candidate)
                        .unwrap()
                        .push(b.clone());

                    continue 'reassign;
                }
            }

            total_valid_ballots -= 1;
        }
    }
}

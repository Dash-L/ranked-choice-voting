#![feature(file_buffered)]

use std::{cell::RefCell, collections::HashMap, env, fs::File, rc::Rc};

use types::{Ballot, rmp_serde::Deserializer, serde::Deserialize};

fn main() {
    let mut args = env::args();
    args.next();

    let vote_data_path = &args.next().unwrap_or("./vote_data.mpack".to_string());
    let vote_data_file = File::open_buffered(vote_data_path).expect(&format!(
        "failed to open vote data file: {}",
        vote_data_path
    ));
    let mut deserializer = Deserializer::new(vote_data_file);

    let mut ballots = Vec::new();
    let mut candidate_votes = HashMap::new();

    while let Ok(mut b) = Ballot::deserialize(&mut deserializer) {
        let mut first_vote = None;
        for i in 0..b.votes.len() {
            if b.votes[i].is_some() {
                first_vote = b.votes[i];
                b.selected = i;
                break;
            }
        }
        if let Some(first_vote) = first_vote {
            let r = Rc::new(RefCell::new(b));
            ballots.push(r.clone());
            candidate_votes
                .entry(first_vote)
                .or_insert(vec![])
                .push(r.clone());
        }
    }

    let mut items = candidate_votes.iter().collect::<Vec<_>>();
    items.sort_by_key(|(_k, v)| v.len());

    for (k, v) in items {
        println!("Candidate {} has {} votes", k, v.len());
    }

    println!("Total valid votes: {}", ballots.len());

    let mut total_valid_ballots = ballots.len();

    loop {
        let mut items = candidate_votes.clone().into_iter().collect::<Vec<_>>();
        items.sort_by_key(|(_k, v)| v.len());

        let (best_id, best_votes) = items.last().unwrap();
        let (worst_id, worst_votes) = items.first().unwrap();

        if best_votes.len() as f64 / total_valid_ballots as f64 > 0.5 {
            println!(
                "The winner is {} with {} votes ({:.3}%)",
                best_id,
                best_votes.len(),
                best_votes.len() as f64 / total_valid_ballots as f64 * 100.
            );

            let (secnd_id, secnd_votes) = items.iter().rev().skip(1).next().unwrap();
            println!(
                "Second is {} with {} votes ({:.3}%)",
                secnd_id,
                secnd_votes.len(),
                secnd_votes.len() as f64 / total_valid_ballots as f64 * 100.
            );
            break;
        }

        println!(
            "The current leader is {} with {} votes ({:.3}%)",
            best_id,
            best_votes.len(),
            best_votes.len() as f64 / total_valid_ballots as f64 * 100.
        );

        candidate_votes.remove(worst_id);

        for b in worst_votes {
            let mut b_ref = b.borrow_mut();
            let mut found_valid_candidate = false;
            for i in (b_ref.selected + 1)..b_ref.votes.len() {
                if let Some(c) = b_ref.votes[i]
                    && candidate_votes.contains_key(&c)
                {
                    b_ref.selected = i;
                    found_valid_candidate = true;
                    break;
                }
            }

            if !found_valid_candidate {
                total_valid_ballots -= 1;
                continue;
            }

            if let Some(next_candidate) = b_ref.votes[b_ref.selected] {
                candidate_votes
                    .get_mut(&next_candidate)
                    .unwrap()
                    .push(b.clone());
            }
        }
    }
}

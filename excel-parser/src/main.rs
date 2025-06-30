#![feature(let_chains, file_buffered)]

use std::{env, fs::File, num::NonZeroUsize, path::Path};

use calamine::{Reader, Xlsx, open_workbook};
use types::{Ballot, rmp_serde::Serializer, serde::Serialize};

const MAGIC_HEADER_ROW_SUFFIXES: &[&str] =
    &["(024306)", "(224306)", "(324306)", "(424306)", "(524306)"];

const MAGIC_HEADER_ROW_PREFIX: &str = "DEM Mayor Choice";

fn main() {
    let mut args = env::args();
    args.next();

    let xlsx_folder_path = args
        .next()
        .expect("Please pass in a path to the directory containing the .xslx vote files");
    let xlsx_folder_path = Path::new(&xlsx_folder_path);

    if !xlsx_folder_path.is_dir() {
        eprintln!("{} is not a directory!", xlsx_folder_path.display());
        return;
    }

    let out_path = &args.next().unwrap_or("./vote_data.mpack".to_string());
    let mut out_file =
        File::create_buffered(out_path).expect(&format!("could not open output file {}", out_path));
    let mut serializer = Serializer::new(&mut out_file);

    let mut total_votes = 0;
    for file in xlsx_folder_path.read_dir().unwrap() {
        if let Ok(file) = file {
            if let Some(fname) = file.file_name().to_str()
                && fname.ends_with(".xlsx")
            {
                println!("Processing file {}", fname);
                let mut local_votes = 0;
                let mut workbook: Xlsx<_> = open_workbook(file.path()).expect("workbook");
                if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
                    let header_indices = range
                        .headers()
                        .unwrap()
                        .iter()
                        .enumerate()
                        .filter_map(|(i, val)| {
                            if val.starts_with(MAGIC_HEADER_ROW_PREFIX) {
                                Some(i)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    if header_indices.len() == 0 {
                        continue;
                    }

                    let mut done = false;
                    for row in range.rows().skip(1) {
                        let mut bad = false;
                        let ballot = Ballot {
                            votes: header_indices
                                .iter()
                                .map(|&idx| match &row[idx] {
                                    calamine::Data::Int(i) => NonZeroUsize::new(*i as usize),
                                    calamine::Data::String(s) => s
                                        .parse::<usize>()
                                        .ok()
                                        .and_then(|n| NonZeroUsize::new(n))
                                        .or(if s == "Write-in" {
                                            NonZeroUsize::new(1)
                                        } else {
                                            None
                                        }),
                                    calamine::Data::Empty => {
                                        done = true;
                                        None
                                    }
                                    _ => None,
                                })
                                .collect(),
                            ..Default::default()
                        };

                        if done {
                            break;
                        }
                        if bad || ballot.votes.iter().all(|v| v.is_none()) {
                            continue;
                        }

                        ballot
                            .serialize(&mut serializer)
                            .expect("serialization failed");

                        local_votes += 1;
                    }

                    println!("Counted {} votes", local_votes);

                    total_votes += local_votes;
                }
            }
        }
    }

    println!("Processed {} votes in total", total_votes);
}

use log::*;
use std::error::Error;

use std::collections::{HashMap, HashSet};

use std::fs::File;
use std::io::{BufRead, BufReader};

/*---------------------------------------------------------------------------*/

fn setup_logger() -> log2::Handle {
    let log_level = match std::env::var("LOG_LEVEL") {
        Ok(val) => val,
        Err(_) => "info".to_string(),
    };
    log2::stdout().module(false).level(log_level).start()
}

fn setup_input() -> std::io::Result<File> {
    let input_path = match std::env::var("INPUT_PATH") {
        Ok(val) => val,
        Err(_) => panic!("Invalid INPUT_PATH. Check if path exists"),
    };
    File::open(&input_path)
}

/*---------------------------------------------------------------------------*/

#[derive(Debug, Default)]
struct Entry {
    before: HashSet<i32>,
    after: HashSet<i32>,
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = setup_logger();
    let reader = BufReader::new(setup_input()?);

    let mut total = 0;
    let mut rules: HashMap<i32, Entry> = HashMap::new();
    let mut empty_line_found = false;
    for line in reader.lines() {
        let line = line?;
        if line == "" {
            trace!("Empty line has been found");
            empty_line_found = true;
            trace!("{:#?}", rules);
            continue;
        }

        if !empty_line_found {
            let v: Vec<i32> = line
                .split("|")
                .map(|x| x.parse::<i32>().unwrap_or(0))
                .collect();

            trace!("Rule: {} | {}", v[0], v[1]);
            if !rules.contains_key(&v[0]) {
                rules.insert(v[0], Default::default());
            }

            if !rules.contains_key(&v[1]) {
                rules.insert(v[1], Default::default());
            }

            rules.entry(v[0]).and_modify(|entry| {
                entry.before.insert(v[1]);
            });

            rules.entry(v[1]).and_modify(|entry| {
                entry.after.insert(v[0]);
            });
        } else {
            let v: Vec<i32> = line
                .split(",")
                .map(|x| x.parse::<i32>().unwrap_or(0))
                .collect();
            trace!("Line to be processed: {:#?}", v);
            let mut valid_line = true;
            for i in 0..v.len() - 1 {
                let value = rules.entry(v[i]).or_default();
                trace!(
                    "First value: {} - Rules {:#?} - Match: {}",
                    v[i],
                    value,
                    value.before.contains(&v[i + 1])
                );
                if !value.before.contains(&v[i + 1]) {
                    valid_line = false;
                    break;
                }
            }
            if valid_line {
                total += v[(v.len() - 1) / 2];
            };
        }
    }

    info!("Day 5 - Exercise 1 result: {}", total);
    Ok(())
}

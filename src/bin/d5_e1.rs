use log::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

#[derive(Debug, Default)]
struct Entry {
    before: HashSet<i32>,
    after: HashSet<i32>,
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);

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

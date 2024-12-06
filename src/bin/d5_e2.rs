use log::*;
use std::error::Error;

use std::collections::{HashMap, HashSet};

use std::fs::File;
use std::io::{BufRead, BufReader};

/*---------------------------------------------------------------------------*/

#[derive(Debug, Default)]
struct Entry {
    before: HashSet<i32>,
    after: HashSet<i32>,
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = log2::stdout().module(false).level("trace").start();

    let mut total = 0;
    let file = File::open("./input/d5.txt")?;
    let reader = BufReader::new(file);

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
            let mut v: Vec<i32> = line
                .split(",")
                .map(|x| x.parse::<i32>().unwrap_or(0))
                .collect();
            trace!("Line to be processed: {:#?}", v);
            let mut valid_line = true;
            for i in 0..v.len() - 1 {
                let value = rules.entry(v[i]).or_default();
                if !value.before.contains(&v[i + 1]) {
                    valid_line = false;
                    let aux = v[i + 1];
                    v[i + 1] = v[i];
                    v[i] = aux;
                }
            }

            trace!("Line after processing: {:#?}", v);
            if !valid_line {
                let mut line_fully_traversed = false;
                while line_fully_traversed_and_valid == false {
                    line_fully_traversed_and_valid = true;
                    for i in 0..v.len() - 1 {
                        trace!("Position: {}", i);
                        let value = rules.entry(v[i]).or_default();
                        if !value.before.contains(&v[i + 1]) {
                            let aux = v[i + 1];
                            v[i + 1] = v[i];
                            v[i] = aux;
                            line_fully_traversed_and_valid = false;
                            break;
                        }
                    }
                }
                trace!("Line after processing: {:#?}", v);
                total += v[(v.len() - 1) / 2];
            };
        }
    }

    info!("Day 5 - Exercise 2 result: {}", total);
    Ok(())
}

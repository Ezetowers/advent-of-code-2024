use log::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn setup_logger() {
    pretty_env_logger::init_timed();
}

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger();

    let file = File::open("./input/d1.txt")?;
    let reader = BufReader::new(file);

    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list_hash: HashMap<i32, i32> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let v: Vec<i32> = line
            .split("   ")
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .collect();
        first_list.push(v[0]);
        second_list_hash
            .entry(v[1])
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let n: usize = first_list.len();
    let mut score = 0;
    for i in 0..n {
        score += first_list[i] * *second_list_hash.entry(first_list[i]).or_default();
    }
    info!("Day 1 - Exercise 2 result: {}", score);
    Ok(())
}

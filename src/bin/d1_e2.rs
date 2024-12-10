use log::*;
use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);

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

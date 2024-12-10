use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);

    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let v: Vec<i32> = line
            .split("   ")
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .collect();
        first_list.push(v[0]);
        second_list.push(v[1]);
    }

    first_list.sort();
    second_list.sort();

    let n: usize = first_list.len();
    let mut sum = 0;
    for i in 0..n {
        let difference = first_list[i] - second_list[i];
        sum += difference.abs();
    }
    info!("Day 1 - Exercise 1 result: {}", sum);
    Ok(())
}

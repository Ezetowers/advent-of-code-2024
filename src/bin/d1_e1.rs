use log::*;
use std::error::Error;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn setup_logger() {
    pretty_env_logger::init_timed();
}

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger();

    let file = File::open("./input/d1_e1.txt")?;
    let reader = BufReader::new(file);

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

use log::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = log2::stdout().module(false).level("trace").start();

    let file = File::open("./input/d7-minimal.txt")?;
    let reader = BufReader::new(file);
    let mut total = 0;

    for line in reader.lines() {
        let line = line?;
        let split_line: Vec<&str> = line.split(":").collect();
        let test_result = split_line[0].parse::<i32>().unwrap_or(0);
        let test_values: Vec<i32> = split_line[1]
            .trim()
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .collect();
        trace!(
            "Test Result: {} - Test Values: {:?}",
            test_result,
            test_values
        );
    }

    info!("Day 7 - Exercise 1. Result: {}", total);
    Ok(())
}

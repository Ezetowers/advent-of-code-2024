use log::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn setup_logger() {
    pretty_env_logger::init_timed();
}

fn validate_level(level: &Vec<i32>) -> bool {
    // We need to make two checks:
    // - The levels are either all increasing or all decreasing.
    // - Any two adjacent levels differ by at least one and at most three.
    let mut increasing = true;
    let mut decreasing = true;
    let mut adjacent_diff = true;
    debug!("Level: {:#?}", level);
    for i in 1..level.len() {
        debug!("Previous: {} - Current: {}", level[i - 1], level[i]);
        let diff = (level[i] - level[i - 1]).abs();
        if (diff < 1) || (diff > 3) {
            debug!(
                "Any two adjacent levels differ by at least one and at most three. Diff: {}",
                diff
            );
            adjacent_diff = false;
            break;
        }

        if increasing == true {
            increasing = level[i] > level[i - 1];
        }
        if decreasing == true {
            decreasing = level[i] < level[i - 1];
        }

        if increasing == false && decreasing == false {
            debug!(
                "Level is not either increasing or decreasing. Increasing: {} - Decreasing: {}",
                increasing, decreasing
            );
            break;
        }
    }
    (increasing == true || decreasing == true) && adjacent_diff == true
}

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger();

    let file = File::open("./input/d2.txt")?;
    let reader = BufReader::new(file);

    let mut valid_levels = 0;
    for line in reader.lines() {
        let line = line?;
        let level: Vec<i32> = line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .collect();

        if validate_level(&level) {
            valid_levels += 1;
        } else {
            // Solution 1: Brute force
            // Check if a level is valid. If it is not, remove one by one every entry in the level
            // and if one of the permutations is valid, stop the analysis
            for i in 0..level.len() {
                let mut new_level = level.clone();
                new_level.remove(i);
                if validate_level(&new_level) {
                    debug!("Level is valid removing element in position {}", i);
                    valid_levels += 1;
                    break;
                }
            }
        }
    }

    info!("[Day 2 - Exercise 1] Result: {}", valid_levels);
    Ok(())
}
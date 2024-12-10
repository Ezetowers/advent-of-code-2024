use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);

    let mut valid_levels = 0;
    for line in reader.lines() {
        let line = line?;
        let level: Vec<i32> = line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .collect();

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
        if (increasing == true || decreasing == true) && adjacent_diff == true {
            valid_levels += 1;
        }
    }

    info!("[Day 2 - Exercise 1] Result: {}", valid_levels);
    Ok(())
}

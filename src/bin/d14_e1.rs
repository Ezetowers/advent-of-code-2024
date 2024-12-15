use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

const WIDE_LIMIT: u32 = 11;
const TALL_LIMIT: u32 = 7;

/*---------------------------------------------------------------------------*/

#[derive(Debug, Default, Copy, Clone)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut total = 0;
    let mut robots: Vec<Robot> = Vec::new();

    for line in reader.lines() {
        total += 1;
        let line = line?;
        let elements: Vec<&str> = line.split(" ").collect();

        let position_string = elements[0].split("=").collect::<Vec<_>>()[1];
        let position: Vec<i32> = position_string
            .split(",")
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .collect();

        let velocity_string = elements[1].split("=").collect::<Vec<_>>()[1];
        let velocity: Vec<i32> = velocity_string
            .split(",")
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .collect();
        robots.push(Robot {
            position: (position[0], position[1]),
            velocity: (velocity[0], velocity[1]),
        });

        trace!("Line: {}", line);
    }

    trace!("Robots input");
    for i in 0..robots.len() {
        trace!("{:?}", robots[i]);
    }

    info!("Day X - Exercise Y. Result: {}", total);
    Ok(())
}

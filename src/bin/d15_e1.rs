use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut total = 0;
    let mut warehouse: Vec<Vec<char>> = Vec::new();
    let mut movements: Vec<char> = Vec::new();

    let mut parse_movement = false;
    for line in reader.lines() {
        // First part, parse the warehouse
        total += 1;
        let line = line?;
        if line.len() == 0 {
            parse_movement = true;
            continue;
        }

        if !parse_movement {
            warehouse.push(line.chars().collect());
        } else {
            for movement in line.chars() {
                movements.push(movement);
            }
        }
    }

    trace!("Warehouse");
    for i in 0..warehouse.len() {
        trace!("{:?}", warehouse[i]);
    }
    trace!("Movements: {:?}", movements);

    info!("Day X - Exercise Y. Result: {}", total);
    Ok(())
}

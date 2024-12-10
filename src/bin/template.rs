use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut total = 0;

    for line in reader.lines() {
        total += 1;
        let line = line?;
        trace!("Line: {}", line);
    }

    info!("Day X - Exercise Y. Result: {}", total);
    Ok(())
}

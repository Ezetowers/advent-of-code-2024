use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut total = 0;
    let mut maze: Vec<Vec<char>> = Vec::new();
    let mut start: (u32, u32) = (0, 0);
    let mut end: (u32, u32) = (0, 0);

    let mut x = 0;
    for line in reader.lines() {
        total += 1;
        let line = line?;
        maze.push(line.chars().collect());

        let mut y = 0;
        for character in line.chars() {
            if character == 'S' {
                start = (x, y);
            } else if character == 'E' {
                end = (x, y);
            }
            y = y + 1;
        }
        x = x + 1;
    }

    trace!("Maze");
    for i in 0..maze.len() {
        let row: String = maze[i].clone().into_iter().collect();
        trace!("{}", row);
    }
    trace!("Start position: {:?}", start);
    trace!("End position: {:?}", end);

    info!("Day X - Exercise Y. Result: {}", total);
    Ok(())
}

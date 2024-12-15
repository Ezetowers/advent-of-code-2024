use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

// const WIDE_LIMIT: i32 = 7;
const WIDE_LIMIT: i32 = 103;
// const TALL_LIMIT: i32 = 11;
const TALL_LIMIT: i32 = 101;

/*---------------------------------------------------------------------------*/

#[derive(Debug, Default, Copy, Clone)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
    ticks: u8,
}

impl Robot {
    fn new(position: (i32, i32), velocity: (i32, i32)) -> Robot {
        Self {
            position,
            velocity,
            ticks: 0,
        }
    }

    fn tick(&mut self) {
        let mut new_position_x = self.position.0 + self.velocity.0;
        if new_position_x < 0 {
            new_position_x = TALL_LIMIT + new_position_x;
        } else {
            new_position_x = (new_position_x % TALL_LIMIT).abs();
        }

        let mut new_position_y = self.position.1 + self.velocity.1;
        if new_position_y < 0 {
            new_position_y = WIDE_LIMIT + new_position_y;
        } else {
            new_position_y = (new_position_y % WIDE_LIMIT).abs();
        }

        self.position = (new_position_x, new_position_y);
        self.ticks += 1;
    }
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut robots: Vec<Robot> = Vec::new();

    for line in reader.lines() {
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
        robots.push(Robot::new(
            (position[0], position[1]),
            (velocity[0], velocity[1]),
        ));

        trace!("Line: {}", line);
    }

    trace!("Robots input");
    for i in 0..robots.len() {
        trace!("{:?}", robots[i]);
    }

    // NOTE: I found the answer to this exercise in a visual way. First I detected that
    // at certain positions, a kind of column was displayed. This kind of column was
    // found at positions 3, 104, 205, etc.
    // With this idea in mind, I only printed these iterations and found that in iteration
    // 6770, the tree was displayed. Since I started the count in 0, the amount of ticks
    // associated with the tree is 6771

    // Move those robots
    for index in 0..10000 {
        let mut grid = [['-'; TALL_LIMIT as usize]; WIDE_LIMIT as usize];
        for i in 0..robots.len() {
            robots[i].tick();
            trace!("[Index {}] {:?}", i, robots[i]);
        }
        // Create the grid
        for i in 0..robots.len() {
            if (index - 3) % 101 == 0 {
                grid[robots[i].position.1 as usize][robots[i].position.0 as usize] = 'X';
            }
        }

        if (index - 3) % 101 != 0 {
            continue;
        }
        // Print the grid
        info!("Tick {}", index + 1);
        for x in 0..WIDE_LIMIT as usize {
            let mut row_as_string: String = String::new();
            for y in 0..TALL_LIMIT as usize {
                row_as_string.push(grid[x][y]);
            }
            debug!("{:?}", row_as_string);
        }
    }

    Ok(())
}

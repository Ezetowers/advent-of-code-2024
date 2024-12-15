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

    // Move those robots
    for _ in 0..100 {
        for i in 0..robots.len() {
            robots[i].tick();
            trace!("[Index {}] {:?}", i, robots[i]);
        }
    }

    // Create the grid
    let mut grid = [[0; TALL_LIMIT as usize]; WIDE_LIMIT as usize];
    for i in 0..robots.len() {
        grid[robots[i].position.1 as usize][robots[i].position.0 as usize] += 1;
    }
    for x in 0..WIDE_LIMIT as usize {
        debug!("{:?}", grid[x]);
    }

    // Analyze the quadrants
    // -----------------
    // |       |       |
    // |   1   |   2   |
    // |       |       |
    // -----------------
    // |       |       |
    // |   3   |   4   |
    // |       |       |
    // -----------------
    let mut quadrants = vec![0; 4];

    // Quadrant 1
    for x in 0..=WIDE_LIMIT / 2 - 1 {
        for y in 0..=TALL_LIMIT / 2 - 1 {
            quadrants[0] += grid[x as usize][y as usize];
        }
    }

    // Quadrant 2
    for x in 0..=WIDE_LIMIT / 2 - 1 {
        for y in TALL_LIMIT / 2 + 1..TALL_LIMIT {
            quadrants[1] += grid[x as usize][y as usize];
        }
    }

    // Quadrant 3
    for x in WIDE_LIMIT / 2 + 1..WIDE_LIMIT {
        for y in 0..=TALL_LIMIT / 2 - 1 {
            quadrants[2] += grid[x as usize][y as usize];
        }
    }

    // Quadrant 4
    for x in WIDE_LIMIT / 2 + 1..WIDE_LIMIT {
        for y in TALL_LIMIT / 2 + 1..TALL_LIMIT {
            quadrants[3] += grid[x as usize][y as usize];
        }
    }

    debug!("Quadrants: {:?}", quadrants);
    total = quadrants[0];
    for i in 1..quadrants.len() {
        total = total * quadrants[i];
    }
    info!("Day 14 - Exercise 2. Result: {}", total);
    Ok(())
}

use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

const X_WEIGHT: usize = 100;
const Y_WEIGHT: usize = 1;

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut total = 0;
    let mut warehouse: Vec<Vec<char>> = Vec::new();
    let mut movements: Vec<char> = Vec::new();
    let mut robot_position: (i32, i32) = (0, 0);

    let mut parse_movement = false;
    let mut x = 0;
    for line in reader.lines() {
        // First part, parse the warehouse
        let line = line?;
        if line.len() == 0 {
            parse_movement = true;
            continue;
        }

        if !parse_movement {
            let mut y = 0;
            for character in line.chars() {
                if character == '@' {
                    robot_position = (x, y);
                }

                y += 1;
            }
            warehouse.push(line.chars().collect());
            x = x + 1;
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
    trace!("Robot initial position: {:?}", robot_position);

    // Done with parsing, starting exercise
    let mut move_number = 0;
    for movement in movements {
        trace!("Next move: {}", movement);
        match movement {
            '>' => {
                trace!(
                    "Position to Robot's right position: {}",
                    warehouse[robot_position.0 as usize][robot_position.1 as usize + 1]
                );
                match warehouse[robot_position.0 as usize][robot_position.1 as usize + 1] {
                    '.' => {
                        warehouse[robot_position.0 as usize][robot_position.1 as usize] = '.';
                        warehouse[robot_position.0 as usize][robot_position.1 as usize + 1] = '@';
                        robot_position.1 += 1;
                    }
                    'O' => {
                        let next_x: i32 = robot_position.0;
                        let mut next_y: i32 = robot_position.1 + 1;
                        while warehouse[next_x as usize][next_y as usize] == 'O' {
                            next_y = next_y + 1;
                        }
                        if warehouse[next_x as usize][next_y as usize] == '.' {
                            trace!(". found. Move boulders");
                            warehouse[next_x as usize][robot_position.1 as usize] = '.';
                            warehouse[next_x as usize][robot_position.1 as usize + 1] = '@';
                            warehouse[next_x as usize][next_y as usize] = 'O';
                            robot_position.1 += 1;
                        }
                    }
                    '#' => {}
                    _ => panic!("This should not happen"),
                }
            }
            '^' => {
                trace!(
                    "Position to Robot's up position: {}",
                    warehouse[robot_position.0 as usize - 1][robot_position.1 as usize]
                );
                match warehouse[robot_position.0 as usize - 1][robot_position.1 as usize] {
                    '.' => {
                        warehouse[robot_position.0 as usize][robot_position.1 as usize] = '.';
                        warehouse[robot_position.0 as usize - 1][robot_position.1 as usize] = '@';
                        robot_position.0 -= 1;
                    }
                    'O' => {
                        let mut next_x: i32 = robot_position.0 - 1;
                        let next_y: i32 = robot_position.1;
                        while warehouse[next_x as usize][next_y as usize] == 'O' {
                            next_x = next_x - 1;
                        }
                        if warehouse[next_x as usize][next_y as usize] == '.' {
                            trace!(". found. Move boulders");
                            warehouse[robot_position.0 as usize][next_y as usize] = '.';
                            warehouse[robot_position.0 as usize - 1][next_y as usize] = '@';
                            warehouse[next_x as usize][next_y as usize] = 'O';
                            robot_position.0 -= 1;
                        }
                    }
                    '#' => {}
                    _ => panic!("This should not happen"),
                }
            }
            '<' => {
                trace!(
                    "Position to Robot's left position: {}",
                    warehouse[robot_position.0 as usize][robot_position.1 as usize - 1]
                );
                match warehouse[robot_position.0 as usize][robot_position.1 as usize - 1] {
                    '.' => {
                        warehouse[robot_position.0 as usize][robot_position.1 as usize] = '.';
                        warehouse[robot_position.0 as usize][robot_position.1 as usize - 1] = '@';
                        robot_position.1 -= 1;
                    }
                    'O' => {
                        let next_x: i32 = robot_position.0;
                        let mut next_y: i32 = robot_position.1 - 1;
                        while warehouse[next_x as usize][next_y as usize] == 'O' {
                            next_y = next_y - 1;
                        }
                        if warehouse[next_x as usize][next_y as usize] == '.' {
                            trace!(". found. Move boulders");
                            warehouse[next_x as usize][robot_position.1 as usize] = '.';
                            warehouse[next_x as usize][robot_position.1 as usize - 1] = '@';
                            warehouse[next_x as usize][next_y as usize] = 'O';
                            robot_position.1 -= 1;
                        }
                    }
                    '#' => {}
                    _ => panic!("This should not happen"),
                }
            }
            'v' => {
                trace!(
                    "Position to Robot's down position: {}",
                    warehouse[robot_position.0 as usize + 1][robot_position.1 as usize]
                );
                match warehouse[robot_position.0 as usize + 1][robot_position.1 as usize] {
                    '.' => {
                        warehouse[robot_position.0 as usize][robot_position.1 as usize] = '.';
                        warehouse[robot_position.0 as usize + 1][robot_position.1 as usize] = '@';
                        robot_position.0 += 1;
                    }
                    'O' => {
                        let mut next_x: i32 = robot_position.0 + 1;
                        let next_y: i32 = robot_position.1;
                        while warehouse[next_x as usize][next_y as usize] == 'O' {
                            next_x = next_x + 1;
                        }
                        if warehouse[next_x as usize][next_y as usize] == '.' {
                            trace!(". found. Move boulders");
                            warehouse[robot_position.0 as usize][next_y as usize] = '.';
                            warehouse[robot_position.0 as usize + 1][next_y as usize] = '@';
                            warehouse[next_x as usize][next_y as usize] = 'O';
                            robot_position.0 += 1;
                        }
                    }
                    '#' => {}
                    _ => panic!("This should not happen"),
                }
            }
            _ => panic!("This should not happen"),
        }
        move_number += 1;
        trace!("Robot position after move: {:?}", robot_position);
        trace!("Warehouse with move {}", move_number);
        for i in 0..warehouse.len() {
            trace!("{:?}", warehouse[i]);
        }
    }

    // NOTE: The warehouse is a square. Change this code to get actual
    // dimensions of the warehouse if needed
    for x in 0..warehouse.len() {
        for y in 0..warehouse.len() {
            if warehouse[x][y] == 'O' {
                total += X_WEIGHT * x + Y_WEIGHT * y;
            }
        }
    }

    info!("Day X - Exercise Y. Result: {}", total);
    Ok(())
}

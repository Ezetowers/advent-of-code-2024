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
    let mut old_warehouse: Vec<Vec<char>> = Vec::new();
    let mut movements: Vec<char> = Vec::new();
    let mut old_robot_position: (i32, i32) = (0, 0);

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
                    old_robot_position = (x, y);
                }

                y += 1;
            }
            old_warehouse.push(line.chars().collect());
            x = x + 1;
        } else {
            for movement in line.chars() {
                movements.push(movement);
            }
        }
    }

    trace!("Warehouse before transformation");
    for i in 0..old_warehouse.len() {
        trace!("{:?}", old_warehouse[i]);
    }

    let mut warehouse: Vec<Vec<char>> = Vec::new();
    for x in 0..old_warehouse.len() {
        let mut row: Vec<char> = Vec::new();
        for y in 0..old_warehouse.len() {
            if x == old_robot_position.0 as usize && y == old_robot_position.1 as usize {
                row.push('@');
                row.push('.');
            } else {
                if old_warehouse[x][y] == 'O' {
                    row.push('[');
                    row.push(']');
                } else {
                    row.push(old_warehouse[x][y]);
                    row.push(old_warehouse[x][y]);
                }
            }
        }
        warehouse.push(row);
    }

    trace!("New Warehouse");
    let mut robot_position: (i32, i32) = (0, 0);
    for x in 0..warehouse.len() {
        trace!("{:?}", warehouse[x]);
        for y in 0..warehouse[x].len() {
            if warehouse[x][y] == '@' {
                robot_position = (x as i32, y as i32);
            }
        }
    }

    trace!("Movements: {:?}", movements);
    trace!("New robot initial position: {:?}", robot_position);

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
                    '[' => {
                        let next_x: i32 = robot_position.0;
                        let mut next_y: i32 = robot_position.1 + 1;
                        while warehouse[next_x as usize][next_y as usize] == '[' {
                            next_y = next_y + 2;
                        }
                        if warehouse[next_x as usize][next_y as usize] == '.' {
                            trace!(". found. Move boulders");
                            warehouse[next_x as usize][robot_position.1 as usize] = '.';
                            for i in (next_y - robot_position.1)..=0 {
                                warehouse[next_x as usize]
                                    [robot_position.1 as usize - i as usize] = warehouse
                                    [next_x as usize]
                                    [robot_position.1 as usize - i as usize - 1];
                            }
                            robot_position.1 += 1;
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
                    ']' => {
                        let next_x: i32 = robot_position.0;
                        let mut next_y: i32 = robot_position.1 - 1;
                        while warehouse[next_x as usize][next_y as usize] == ']' {
                            next_y = next_y - 2;
                        }
                        if warehouse[next_x as usize][next_y as usize] == '.' {
                            trace!(". found. Move boulders");
                            for i in (next_y - robot_position.1)..=0 {
                                warehouse[next_x as usize]
                                    [robot_position.1 as usize + i as usize] = warehouse
                                    [next_x as usize]
                                    [robot_position.1 as usize + i as usize + 1];
                            }
                            robot_position.1 -= 1;
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

    info!("Day 15 - Exercise 2. Result: {}", total);
    Ok(())
}

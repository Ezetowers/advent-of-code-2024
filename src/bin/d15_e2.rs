use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

// FIXME: There is a lot of duplicated code in this exercise, but I am not going to modify it
// (at least not now) since I want to continue with the following exercises. Things that I
// think can be modified to decrease the amount of code
// * Right (>) and Left (<) movements are basically the same, the only thing that change is
//  Y-axis, so I function can be created using as a parameter the sign of the increase/decrease
//  in the Y-axis
// * Same with the Up (^) and Down (^) positions, but in the X-Axis instead
// Those changes should reduce the code in half

const X_WEIGHT: usize = 100;
const Y_WEIGHT: usize = 1;

fn try_move(warehouse: &Vec<Vec<char>>, pos: (i32, i32), direction: char) -> bool {
    trace!(
        "try_move: Element: {} - pos: {:?}",
        warehouse[pos.0 as usize][pos.1 as usize],
        pos
    );
    let should_move;
    if direction == '^' {
        match warehouse[pos.0 as usize - 1][pos.1 as usize] {
            '[' => {
                should_move = try_move(warehouse, (pos.0 - 1, pos.1), direction)
                    && try_move(warehouse, (pos.0 - 1, pos.1 + 1), direction);
            }
            ']' => {
                should_move = try_move(warehouse, (pos.0 - 1, pos.1), direction)
                    && try_move(warehouse, (pos.0 - 1, pos.1 - 1), direction);
            }
            '#' => {
                should_move = false;
            }
            '.' => {
                should_move = true;
            }
            _ => panic!("try_move: This should not happen"),
        }
    } else if direction == 'v' {
        match warehouse[pos.0 as usize + 1][pos.1 as usize] {
            '[' => {
                should_move = try_move(warehouse, (pos.0 + 1, pos.1), direction)
                    && try_move(warehouse, (pos.0 + 1, pos.1 + 1), direction);
            }
            ']' => {
                should_move = try_move(warehouse, (pos.0 + 1, pos.1), direction)
                    && try_move(warehouse, (pos.0 + 1, pos.1 - 1), direction);
            }
            '#' => {
                should_move = false;
            }
            '.' => {
                should_move = true;
            }
            _ => panic!("try_move: This should not happen",),
        }
    } else {
        panic!("try_move: This should not happen");
    }

    trace!(
        "try_move: Element: {} - pos: {:?} - Should move: {}",
        warehouse[pos.0 as usize][pos.1 as usize],
        pos,
        should_move,
    );
    should_move
}

fn make_move(warehouse: &mut Vec<Vec<char>>, pos: (i32, i32), direction: char) {
    let element = warehouse[pos.0 as usize][pos.1 as usize];
    trace!("make_move: Element: {} - pos: {:?}", element, pos);
    if direction == '^' {
        match warehouse[pos.0 as usize - 1][pos.1 as usize] {
            '[' => {
                make_move(warehouse, (pos.0 - 1, pos.1), direction);
                warehouse[pos.0 as usize - 1][pos.1 as usize] =
                    warehouse[pos.0 as usize][pos.1 as usize];
                if element == ']' {
                    make_move(warehouse, (pos.0 - 1, pos.1 + 1), direction);
                    warehouse[pos.0 as usize - 1][pos.1 as usize + 1] = '.';
                }
            }
            ']' => {
                make_move(warehouse, (pos.0 - 1, pos.1), direction);
                warehouse[pos.0 as usize - 1][pos.1 as usize] =
                    warehouse[pos.0 as usize][pos.1 as usize];
                if element == '[' {
                    make_move(warehouse, (pos.0 - 1, pos.1 - 1), direction);
                    warehouse[pos.0 as usize - 1][pos.1 as usize - 1] = '.';
                }
            }
            '.' => {
                warehouse[pos.0 as usize - 1][pos.1 as usize] =
                    warehouse[pos.0 as usize][pos.1 as usize];
            }
            '#' => panic!("make_move: This should not happen"),
            _ => panic!("make_move: This should not happen"),
        }
    } else if direction == 'v' {
        match warehouse[pos.0 as usize + 1][pos.1 as usize] {
            '[' => {
                make_move(warehouse, (pos.0 + 1, pos.1), direction);
                warehouse[pos.0 as usize + 1][pos.1 as usize] =
                    warehouse[pos.0 as usize][pos.1 as usize];
                if element == ']' {
                    make_move(warehouse, (pos.0 + 1, pos.1 + 1), direction);
                    warehouse[pos.0 as usize + 1][pos.1 as usize + 1] = '.';
                }
            }
            ']' => {
                make_move(warehouse, (pos.0 + 1, pos.1), direction);
                warehouse[pos.0 as usize + 1][pos.1 as usize] =
                    warehouse[pos.0 as usize][pos.1 as usize];
                if element == '[' {
                    make_move(warehouse, (pos.0 + 1, pos.1 - 1), direction);
                    warehouse[pos.0 as usize + 1][pos.1 as usize - 1] = '.';
                }
            }
            '.' => {
                warehouse[pos.0 as usize + 1][pos.1 as usize] =
                    warehouse[pos.0 as usize][pos.1 as usize];
            }
            '#' => panic!("make_move: This should not happen"),
            _ => panic!("make_move: This should not happen"),
        }
    } else {
        panic!("make_move: This should not happen");
    }
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut total = 0;
    let mut old_warehouse: Vec<Vec<char>> = Vec::new();
    let mut movements: Vec<char> = Vec::new();
    let mut old_rob_pos: (i32, i32) = (0, 0);

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
                    old_rob_pos = (x, y);
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
        let row: String = old_warehouse[i].clone().into_iter().collect();
        trace!("{}", row);
    }

    let mut warehouse: Vec<Vec<char>> = Vec::new();
    for x in 0..old_warehouse.len() {
        let mut row: Vec<char> = Vec::new();
        for y in 0..old_warehouse.len() {
            if x == old_rob_pos.0 as usize && y == old_rob_pos.1 as usize {
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
    let mut rob_pos: (i32, i32) = (0, 0);
    for x in 0..warehouse.len() {
        let row: String = warehouse[x].clone().into_iter().collect();
        trace!("{}", row);
        for y in 0..warehouse[x].len() {
            if warehouse[x][y] == '@' {
                rob_pos = (x as i32, y as i32);
            }
        }
    }

    trace!("Movements: {:?}", movements);
    trace!("New robot initial pos: {:?}", rob_pos);

    // Done with parsing, starting exercise
    let mut move_number = 0;
    for movement in movements {
        trace!("Next move: {}", movement);
        match movement {
            '>' => {
                trace!(
                    "pos to Robot's right pos: {}",
                    warehouse[rob_pos.0 as usize][rob_pos.1 as usize + 1]
                );
                match warehouse[rob_pos.0 as usize][rob_pos.1 as usize + 1] {
                    '.' => {
                        warehouse[rob_pos.0 as usize][rob_pos.1 as usize] = '.';
                        warehouse[rob_pos.0 as usize][rob_pos.1 as usize + 1] = '@';
                        rob_pos.1 += 1;
                    }
                    '[' => {
                        let next_x: i32 = rob_pos.0;
                        let mut next_y: i32 = rob_pos.1 + 1;
                        while warehouse[next_x as usize][next_y as usize] == '[' {
                            next_y = next_y + 2;
                        }
                        if warehouse[next_x as usize][next_y as usize] == '.' {
                            trace!(". found. Move boulders");
                            for i in (rob_pos.1..=next_y).rev() {
                                trace!(
                                    "I: {} - Left: {} - Right: {}",
                                    i,
                                    warehouse[next_x as usize][i as usize],
                                    warehouse[next_x as usize][i as usize - 1]
                                );
                                warehouse[next_x as usize][i as usize] =
                                    warehouse[next_x as usize][i as usize - 1];
                            }
                            warehouse[next_x as usize][rob_pos.1 as usize] = '.';
                            rob_pos.1 += 1;
                        }
                    }
                    '#' => {}
                    _ => panic!("This should not happen"),
                }
            }
            '<' => {
                trace!(
                    "pos to Robot's left pos: {}",
                    warehouse[rob_pos.0 as usize][rob_pos.1 as usize - 1]
                );
                match warehouse[rob_pos.0 as usize][rob_pos.1 as usize - 1] {
                    '.' => {
                        warehouse[rob_pos.0 as usize][rob_pos.1 as usize] = '.';
                        warehouse[rob_pos.0 as usize][rob_pos.1 as usize - 1] = '@';
                        rob_pos.1 -= 1;
                    }
                    ']' => {
                        let next_x: i32 = rob_pos.0;
                        let mut next_y: i32 = rob_pos.1 - 1;
                        while warehouse[next_x as usize][next_y as usize] == ']' {
                            next_y = next_y - 2;
                        }
                        if warehouse[next_x as usize][next_y as usize] == '.' {
                            trace!(". found. Move boulders");
                            for i in next_y..=rob_pos.1 {
                                warehouse[next_x as usize][i as usize] =
                                    warehouse[next_x as usize][i as usize + 1];
                            }
                            warehouse[next_x as usize][rob_pos.1 as usize] = '.';
                            rob_pos.1 -= 1;
                        }
                    }
                    '#' => {}
                    _ => panic!("This should not happen"),
                }
            }
            '^' => {
                trace!(
                    "pos to Robot's up pos: {}",
                    warehouse[rob_pos.0 as usize - 1][rob_pos.1 as usize]
                );
                match warehouse[rob_pos.0 as usize - 1][rob_pos.1 as usize] {
                    '.' => {
                        warehouse[rob_pos.0 as usize][rob_pos.1 as usize] = '.';
                        warehouse[rob_pos.0 as usize - 1][rob_pos.1 as usize] = '@';
                        rob_pos.0 -= 1;
                    }
                    '[' => {
                        let should_move = try_move(&warehouse, (rob_pos.0 - 1, rob_pos.1), '^')
                            && try_move(&warehouse, (rob_pos.0 - 1, rob_pos.1 + 1), '^');

                        if should_move {
                            make_move(&mut warehouse, (rob_pos.0 - 1, rob_pos.1), '^');
                            make_move(&mut warehouse, (rob_pos.0 - 1, rob_pos.1 + 1), '^');
                            warehouse[rob_pos.0 as usize][rob_pos.1 as usize] = '.';
                            warehouse[rob_pos.0 as usize - 1][rob_pos.1 as usize] = '@';
                            warehouse[rob_pos.0 as usize - 1][rob_pos.1 as usize + 1] = '.';
                            rob_pos.0 -= 1;
                        }
                    }
                    ']' => {
                        let should_move = try_move(&warehouse, (rob_pos.0 - 1, rob_pos.1), '^')
                            && try_move(&warehouse, (rob_pos.0 - 1, rob_pos.1 - 1), '^');
                        if should_move {
                            make_move(&mut warehouse, (rob_pos.0 - 1, rob_pos.1), '^');
                            make_move(&mut warehouse, (rob_pos.0 - 1, rob_pos.1 - 1), '^');
                            warehouse[rob_pos.0 as usize][rob_pos.1 as usize] = '.';
                            warehouse[rob_pos.0 as usize - 1][rob_pos.1 as usize] = '@';
                            warehouse[rob_pos.0 as usize - 1][rob_pos.1 as usize - 1] = '.';
                            rob_pos.0 -= 1;
                        }
                    }

                    '#' => {}
                    _ => panic!("This should not happen"),
                }
            }
            'v' => {
                trace!(
                    "pos to Robot's down pos: {}",
                    warehouse[rob_pos.0 as usize + 1][rob_pos.1 as usize]
                );
                match warehouse[rob_pos.0 as usize + 1][rob_pos.1 as usize] {
                    '.' => {
                        warehouse[rob_pos.0 as usize][rob_pos.1 as usize] = '.';
                        warehouse[rob_pos.0 as usize + 1][rob_pos.1 as usize] = '@';
                        rob_pos.0 += 1;
                    }
                    '[' => {
                        let should_move = try_move(&warehouse, (rob_pos.0 + 1, rob_pos.1), 'v')
                            && try_move(&warehouse, (rob_pos.0 + 1, rob_pos.1 + 1), 'v');

                        if should_move {
                            make_move(&mut warehouse, (rob_pos.0 + 1, rob_pos.1), 'v');
                            make_move(&mut warehouse, (rob_pos.0 + 1, rob_pos.1 + 1), 'v');
                            warehouse[rob_pos.0 as usize][rob_pos.1 as usize] = '.';
                            warehouse[rob_pos.0 as usize + 1][rob_pos.1 as usize] = '@';
                            warehouse[rob_pos.0 as usize + 1][rob_pos.1 as usize + 1] = '.';
                            rob_pos.0 += 1;
                        }
                    }
                    ']' => {
                        let should_move = try_move(&warehouse, (rob_pos.0 + 1, rob_pos.1), 'v')
                            && try_move(&warehouse, (rob_pos.0 + 1, rob_pos.1 - 1), 'v');
                        if should_move {
                            make_move(&mut warehouse, (rob_pos.0 + 1, rob_pos.1), 'v');
                            make_move(&mut warehouse, (rob_pos.0 + 1, rob_pos.1 - 1), 'v');
                            warehouse[rob_pos.0 as usize][rob_pos.1 as usize] = '.';
                            warehouse[rob_pos.0 as usize + 1][rob_pos.1 as usize] = '@';
                            warehouse[rob_pos.0 as usize + 1][rob_pos.1 as usize - 1] = '.';
                            rob_pos.0 += 1;
                        }
                    }
                    '#' => {}
                    _ => panic!("This should not happen"),
                }
            }
            _ => panic!("This should not happen"),
        }
        move_number += 1;
        trace!("Robot pos after move: {:?}", rob_pos);
        trace!("Warehouse with move {}", move_number);
        for i in 0..warehouse.len() {
            let row: String = warehouse[i].clone().into_iter().collect();
            trace!("{}", row);
        }
    }

    for x in 0..warehouse.len() {
        for y in 0..warehouse[x].len() {
            if warehouse[x][y] == '[' {
                total += X_WEIGHT * x + Y_WEIGHT * y;
            }
        }
    }

    info!("Day 15 - Exercise 2. Result: {}", total);
    Ok(())
}

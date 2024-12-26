use log::*;
use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader};

use pathfinding::prelude::dijkstra;

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

const TURN_WEIGHT: u32 = 1000;
const ADVANCE_WEIGHT: u32 = 1;

/*---------------------------------------------------------------------------*/

#[derive(Default, Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct State {
    pos: (usize, usize),
    direction: (i32, i32),
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut maze: Vec<Vec<char>> = Vec::new();
    let mut start_state: State = Default::default();
    let mut end_state: State = Default::default();

    let mut x = 0;

    for line in reader.lines() {
        let line = line?;
        maze.push(line.chars().collect());

        let mut y = 0;
        for character in line.chars() {
            if character == 'S' {
                start_state = State {
                    pos: (x, y),
                    direction: (0, 1),
                };
                maze[x][y] = '.';
            }
            if character == 'E' {
                end_state = State {
                    pos: (x, y),
                    direction: (0, 0),
                };
                maze[x][y] = '.';
            }
            y = y + 1;
        }
        x = x + 1;
    }

    let result = dijkstra(
        &start_state,
        |state| {
            let mut result: Vec<(State, u32)> = Vec::new();
            let up = maze[state.pos.0 - 1][state.pos.1] == '.';
            let down = maze[state.pos.0 + 1][state.pos.1] == '.';
            let left = maze[state.pos.0][state.pos.1 - 1] == '.';
            let right = maze[state.pos.0][state.pos.1 + 1] == '.';

            if right {
                // Left
                if state.direction.0 == 0 && state.direction.1 == -1 {
                    // Do nothing
                } else {
                    let mut cost = ADVANCE_WEIGHT;
                    let new_state = State {
                        pos: (state.pos.0, state.pos.1 + 1),
                        direction: (0, 1),
                    };

                    if state.direction.0 != 0 || state.direction.1 != 1 {
                        cost += TURN_WEIGHT;
                    }
                    result.push((new_state, cost));
                }
            }

            if left {
                // Right
                if state.direction.0 == 0 && state.direction.1 == 1 {
                    // Do nothing
                } else {
                    let mut cost = ADVANCE_WEIGHT;
                    let new_state = State {
                        pos: (state.pos.0, state.pos.1 - 1),
                        direction: (0, -1),
                    };

                    if state.direction.0 != 0 || state.direction.1 != -1 {
                        cost += TURN_WEIGHT;
                    }
                    result.push((new_state, cost));
                }
            }

            if up {
                // Down
                if state.direction.0 == 1 && state.direction.1 == 0 {
                    // Do nothing
                } else {
                    let mut cost = ADVANCE_WEIGHT;
                    let new_state = State {
                        pos: (state.pos.0 - 1, state.pos.1),
                        direction: (-1, 0),
                    };

                    if state.direction.0 != -1 || state.direction.1 != 0 {
                        cost += TURN_WEIGHT;
                    }
                    result.push((new_state, cost));
                }
            }

            if down {
                // Up
                if state.direction.0 == -1 && state.direction.1 == 0 {
                    // Do nothing
                } else {
                    let mut cost = ADVANCE_WEIGHT;
                    let new_state = State {
                        pos: (state.pos.0 + 1, state.pos.1),
                        direction: (1, 0),
                    };

                    if state.direction.0 != 1 || state.direction.1 != 0 {
                        cost += TURN_WEIGHT;
                    }
                    result.push((new_state, cost));
                }
            }

            result
        },
        |state| state.pos == end_state.pos,
    );

    match result {
        Some(result) => info!("Day 16 - Exercise 1. Result: {:#?}", result.1),
        None => info!("Day 16 - Exercise 1. Result: None"),
    };
    Ok(())
}

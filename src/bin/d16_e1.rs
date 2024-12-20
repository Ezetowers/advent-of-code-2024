use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use uuid::Uuid;

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

const TURN_WEIGHT: u32 = 1000;
const ADVANCE_WEIGHT: u32 = 1;
const WALL_CHAR: char = 'O';

/*---------------------------------------------------------------------------*/

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    UP,
    #[default]
    RIGHT,
    LEFT,
    DOWN,
}

#[derive(Debug, Default, Clone)]
struct Maze {
    uuid: Uuid,
    current_pos: (usize, usize),
    end_pos: (usize, usize),
    direction: Direction,
    score: u32,
    grid: Vec<Vec<char>>,
    paths_weights: Vec<u32>,
}

impl Maze {
    fn new(start_pos: (usize, usize), end_pos: (usize, usize), grid: Vec<Vec<char>>) -> Self {
        Self {
            current_pos: start_pos,
            end_pos,
            grid: grid.clone(),
            score: 0,
            direction: Direction::RIGHT,
            paths_weights: Vec::new(),
            uuid: Uuid::new_v4(),
        }
    }

    fn dir_char(&self) -> char {
        match self.direction {
            Direction::RIGHT => return '>',
            Direction::LEFT => return '<',
            Direction::UP => return '^',
            Direction::DOWN => return 'v',
        }
    }

    fn solve(&mut self) {
        loop {
            let up_continue = self.grid[self.current_pos.0 - 1][self.current_pos.1] == '.';
            let down_continue = self.grid[self.current_pos.0 + 1][self.current_pos.1] == '.';
            let left_continue = self.grid[self.current_pos.0][self.current_pos.1 - 1] == '.';
            let right_continue = self.grid[self.current_pos.0][self.current_pos.1 + 1] == '.';

            trace!("[ID {}] Maze", self.uuid);
            for i in 0..self.grid.len() {
                let row: String = self.grid[i].clone().into_iter().collect();
                trace!("{}", row);
            }

            trace!(
                "[ID {}] Current position: {:?} - Direction: {:?} - Score: {} - Up: {} - Down: {} - Left: {} - Right: {}",
                self.uuid,
                self.current_pos,
                self.direction,
                self.score,
                up_continue,
                down_continue,
                left_continue,
                right_continue,
            );

            // Use case 1: We reach the final position
            let mut next_pos = self.current_pos;
            match self.direction {
                Direction::UP => next_pos.0 -= 1,
                Direction::DOWN => next_pos.0 += 1,
                Direction::LEFT => next_pos.1 -= 1,
                Direction::RIGHT => next_pos.1 += 1,
            }
            if next_pos.0 == self.end_pos.0 && next_pos.1 == self.end_pos.1 {
                self.paths_weights.push(self.score);
                trace!(
                    "[ID {}] End of maze found!. Final score: {}",
                    self.uuid,
                    self.score
                );
                break;
            }

            // Use case 2: Dead end found
            if !up_continue && !down_continue && !left_continue && !right_continue {
                trace!("[ID {}] Dead end found", self.uuid);
                break;
            }

            // Mark the current position as visited
            self.grid[self.current_pos.0][self.current_pos.1] = '@';

            // Use case 3: We have only one option to continue
            if right_continue && (!up_continue && !down_continue && !left_continue) {
                trace!("[ID {}] Only right direction found!", self.uuid);
                if self.direction == Direction::RIGHT {
                    self.score += ADVANCE_WEIGHT;
                } else {
                    self.score += TURN_WEIGHT;
                    self.direction = Direction::RIGHT;
                }
                self.grid[self.current_pos.0][self.current_pos.1] = self.dir_char();
                self.current_pos.1 += 1;
                continue;
            }

            if left_continue && (!up_continue && !down_continue && !right_continue) {
                trace!("[ID {}] Only left direction found!", self.uuid);
                if self.direction == Direction::LEFT {
                    self.score += ADVANCE_WEIGHT;
                } else {
                    self.score += TURN_WEIGHT;
                    self.direction = Direction::LEFT;
                }
                self.grid[self.current_pos.0][self.current_pos.1] = self.dir_char();
                self.current_pos.1 -= 1;
                continue;
            }

            if down_continue && (!left_continue && !right_continue && !up_continue) {
                trace!("[ID {}] Only down direction found!", self.uuid);
                if self.direction == Direction::DOWN {
                    self.score += ADVANCE_WEIGHT;
                } else {
                    self.score += TURN_WEIGHT;
                    self.direction = Direction::DOWN;
                }
                self.grid[self.current_pos.0][self.current_pos.1] = self.dir_char();
                self.current_pos.0 += 1;
                continue;
            }
            if up_continue && (!left_continue && !right_continue && !down_continue) {
                trace!("[ID {}] Only up direction found!", self.uuid);
                if self.direction == Direction::UP {
                    self.score += ADVANCE_WEIGHT;
                } else {
                    self.score += TURN_WEIGHT;
                    self.direction = Direction::UP;
                }
                self.grid[self.current_pos.0][self.current_pos.1] = self.dir_char();
                self.current_pos.0 -= 1;
                continue;
            }

            // Use case 4: We have two options to continue. Create a new maze to continue one of
            // the paths, marking the path not taken as already visited (@). Do the same for the
            // other path (current solve function) and iterate to have only one option to visit
            // in the next loop
            let mut maze = self.clone();
            maze.uuid = Uuid::new_v4();
            if right_continue && up_continue {
                maze.grid[self.current_pos.0][self.current_pos.1 + 1] = WALL_CHAR;
                maze.solve();
                self.grid[self.current_pos.0 - 1][self.current_pos.1] = WALL_CHAR;
            }

            if right_continue && down_continue {
                maze.grid[self.current_pos.0][self.current_pos.1 + 1] = WALL_CHAR;
                maze.solve();
                self.grid[self.current_pos.0 + 1][self.current_pos.1] = WALL_CHAR;
            }

            if right_continue && left_continue {
                maze.grid[self.current_pos.0][self.current_pos.1 + 1] = WALL_CHAR;
                maze.solve();
                self.grid[self.current_pos.0][self.current_pos.1 - 1] = WALL_CHAR;
            }

            if up_continue && left_continue {
                maze.grid[self.current_pos.0 - 1][self.current_pos.1] = WALL_CHAR;
                maze.solve();
                self.grid[self.current_pos.0][self.current_pos.1 - 1] = WALL_CHAR;
            }

            if up_continue && down_continue {
                maze.grid[self.current_pos.0 - 1][self.current_pos.1] = WALL_CHAR;
                maze.solve();
                self.grid[self.current_pos.0 + 1][self.current_pos.1] = WALL_CHAR;
            }

            if down_continue && left_continue {
                maze.grid[self.current_pos.0 + 1][self.current_pos.1] = WALL_CHAR;
                maze.solve();
                self.grid[self.current_pos.0][self.current_pos.1 - 1] = WALL_CHAR;
            }

            // for weight in maze.paths_weights.into_iter() {
            //     self.paths_weights.push(weight);
            // }
        }
    }
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut maze: Vec<Vec<char>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    let mut x = 0;
    for line in reader.lines() {
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

    let mut maze = Maze::new(start, end, maze);
    maze.solve();
    let mut shortest_path = maze.paths_weights[0];
    for weight in maze.paths_weights.into_iter() {
        if weight < shortest_path {
            shortest_path = weight;
        }
    }

    info!("Day X - Exercise Y. Result: {}", shortest_path);
    Ok(())
}

use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

const TURN_WEIGHT: u32 = 1000;

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
    current_pos: (usize, usize),
    end_pos: (usize, usize),
    direction: Direction,
    score: u32,
    grid: Vec<Vec<char>>,
}

impl Maze {
    fn new(start_pos: (usize, usize), end_pos: (usize, usize), grid: Vec<Vec<char>>) -> Self {
        Self {
            current_pos: start_pos,
            end_pos,
            grid: grid.clone(),
            score: 0,
            direction: Direction::RIGHT,
        }
    }

    fn solve_maze(&mut self) {
        match self.direction {
            Direction::RIGHT => {
                if self.grid[self.current_pos.0][self.current_pos.1 + 1] == '#' {
                    let up_continue = self.grid[self.current_pos.0 + 1][self.current_pos.1] == '#';
                    let down_continue =
                        self.grid[self.current_pos.0 - 1][self.current_pos.1] == '#';
                    if up_continue && down_continue {
                        trace!(
                            "Current Position: {:?} - Direction: {:?}. Dead end found",
                            self.current_pos,
                            self.direction
                        );
                        return;
                    }
                    if up_continue && !down_continue {
                        self.direction = Direction::UP;
                        self.score += TURN_WEIGHT;
                    }

                    if !up_continue && down_continue {
                        self.direction = Direction::DOWN;
                        self.score += TURN_WEIGHT;
                    }
                }
            }
            Direction::LEFT => todo!(),
            Direction::UP => todo!(),
            Direction::DOWN => todo!(),
        }
    }
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut total = 0;
    let mut maze: Vec<Vec<char>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

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

    let mut maze = Maze::new(start, end, maze);
    maze.solve_maze();

    info!("Day X - Exercise Y. Result: {}", total);
    Ok(())
}

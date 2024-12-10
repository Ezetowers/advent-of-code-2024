use log::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*---------------------------------------------------------------------------*/

fn setup_logger() -> log2::Handle {
    let log_level = match std::env::var("LOG_LEVEL") {
        Ok(val) => val,
        Err(_) => "info".to_string(),
    };
    log2::stdout().module(false).level(log_level).start()
}

fn setup_input() -> std::io::Result<File> {
    let input_path = match std::env::var("INPUT_PATH") {
        Ok(val) => val,
        Err(_) => panic!("Invalid INPUT_PATH. Check if path exists"),
    };
    File::open(&input_path)
}

/*---------------------------------------------------------------------------*/

#[derive(Debug, Default, PartialEq)]
enum Direction {
    UP,
    RIGHT,
    LEFT,
    #[default]
    DOWN,
}

#[derive(Debug, Default, PartialEq)]
struct Guardian {
    x: usize,
    y: usize,
    limit: usize,
    direction: Direction,
}

impl Guardian {
    fn new(x: usize, y: usize, limit: usize, symbol: char) -> Self {
        let direction = match symbol {
            '^' => Direction::UP,
            '<' => Direction::LEFT,
            '>' => Direction::RIGHT,
            'v' => Direction::DOWN,
            _ => panic!("This should not happen"),
        };
        Self {
            x,
            y,
            limit,
            direction,
        }
    }

    fn advance(&mut self) {
        match self.direction {
            Direction::UP => {
                if self.x != 0 {
                    self.x = self.x - 1
                }
            }
            Direction::LEFT => {
                if self.y != 0 {
                    self.y = self.y - 1
                }
            }
            Direction::RIGHT => self.y = self.y + 1,
            Direction::DOWN => self.x = self.x + 1,
        }
    }

    fn next_position(&self) -> Option<(usize, usize)> {
        match self.direction {
            Direction::UP => {
                if self.x != 0 {
                    return Some((self.x - 1, self.y));
                } else {
                    None
                }
            }
            Direction::LEFT => {
                if self.y != 0 {
                    return Some((self.x, self.y - 1));
                } else {
                    None
                }
            }
            Direction::RIGHT => {
                if self.y + 1 != self.limit {
                    Some((self.x, self.y + 1))
                } else {
                    None
                }
            }
            Direction::DOWN => {
                if self.x + 1 != self.limit {
                    Some((self.x + 1, self.y))
                } else {
                    None
                }
            }
        }
    }

    fn turn(&mut self) {
        match self.direction {
            Direction::UP => self.direction = Direction::RIGHT,
            Direction::RIGHT => self.direction = Direction::DOWN,
            Direction::DOWN => self.direction = Direction::LEFT,
            Direction::LEFT => self.direction = Direction::UP,
        }
    }
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = setup_logger();
    let reader = BufReader::new(setup_input()?);

    let mut input: Vec<Vec<char>> = Vec::new();
    let guardian_array = ['^', '>', 'v', '<'];
    let mut guardian_found = false;
    let mut guardian = Guardian::default();

    // NOTE: The input is a square matrix, we just need to get one dimension to
    // to identify the other one
    let mut input_square_matrix_dimension = 0;
    let mut index = 0;
    for line in reader.lines() {
        let line = line?;
        let my_chars: Vec<_> = line.chars().collect();
        if input_square_matrix_dimension == 0 {
            input_square_matrix_dimension = line.len();
        }
        input.push(my_chars.clone());

        if !guardian_found {
            for i in 0..input_square_matrix_dimension {
                for j in 0..guardian_array.len() {
                    if my_chars[i] == guardian_array[j] {
                        trace!("Position: ({},{}) - Guardian has been found", index, i);
                        guardian_found = true;
                        guardian =
                            Guardian::new(index, i, input_square_matrix_dimension, my_chars[i]);
                        break;
                    }
                }
            }
        }
        index += 1;
    }

    trace!("Guardian: {:#?}", guardian);
    let mut out_of_bounds = false;
    input[guardian.x][guardian.y] = 'X';
    while !out_of_bounds {
        trace!("Guardian journey: {:?}", guardian);
        match guardian.next_position() {
            Some((x, y)) => {
                if input[x][y] == '#' {
                    guardian.turn();
                } else {
                    input[x][y] = 'X';
                    guardian.advance();
                }
            }
            None => out_of_bounds = true,
        }
    }

    let mut total = 0;
    for x in 0..input_square_matrix_dimension {
        for y in 0..input_square_matrix_dimension {
            if input[x][y] == 'X' {
                total += 1;
            }
        }
    }

    for i in 0..input_square_matrix_dimension {
        trace!("Matrix: {:?}", input[i]);
    }

    info!("Day 6 - Exercise 1. Result: {}", total);
    Ok(())
}

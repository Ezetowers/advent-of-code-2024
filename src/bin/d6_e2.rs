use log::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*---------------------------------------------------------------------------*/
#[derive(Debug, Default, PartialEq, Copy, Clone)]
enum Direction {
    UP,
    RIGHT,
    LEFT,
    #[default]
    DOWN,
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
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

fn loop_found(input: &Vec<Vec<char>>, original_guardian: &Guardian) -> bool {
    let mut guardian = original_guardian.clone();
    let mut out_of_bounds = false;
    let mut iterations = 0;
    while !out_of_bounds {
        match guardian.next_position() {
            Some((x, y)) => {
                if input[x][y] == '#' {
                    guardian.turn();
                } else {
                    guardian.advance();
                }
            }
            None => out_of_bounds = true,
        }
        iterations += 1;
        if iterations == 10000 {
            // FIXME: Horrible way to detect a loop, but given that the input is a
            // 130x130 matrix, then you can constraint the iterations to the max
            // loop that you can find with such a matrix. Finding this loop is not
            // trivial, but testing different iteration numbers is easy enough
            // 10000 is enough, lower numbers work too
            return true;
        }
    }
    false
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = log2::stdout().module(false).level("info").start();

    let file = File::open("./input/d6.txt")?;
    let reader = BufReader::new(file);
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

    let mut total = 0;
    for x in 0..input_square_matrix_dimension {
        for y in 0..input_square_matrix_dimension {
            if input[x][y] == '.' {
                input[x][y] = '#';
                let loop_found = loop_found(&input, &guardian);
                trace!("Boulder position: [{},{}] - Result: {}", x, y, loop_found);
                input[x][y] = '.';
                if loop_found {
                    total += 1;
                }
            }
        }
    }

    info!("Day 6 - Exercise 2. Result: {}", total);
    Ok(())
}

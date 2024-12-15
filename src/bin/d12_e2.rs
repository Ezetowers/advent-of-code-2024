use log::*;
use std::collections::HashSet;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

#[derive(Debug, Default, Copy, Clone)]
struct Plant {
    plant_type: char,
    visited: bool,
}

#[derive(Debug, Default, Hash, Clone, Eq, PartialEq)]
struct Element {
    x: usize,
    y: usize,
    adjacents: Vec<Direction>,
    amount_links: usize,
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    UP,
    #[default]
    RIGHT,
    LEFT,
    DOWN,
}

#[derive(Debug, Default, Copy, Clone)]
struct Arrow {
    position: (usize, usize),
    direction: Direction,
    turns: u32,
}

impl Arrow {
    fn new(x: usize, y: usize) -> Self {
        Self {
            position: (x, y),
            direction: Direction::RIGHT,
            turns: 0,
        }
    }

    fn turn(&mut self, grid: &[[i32; 30]; 30]) {
        match self.direction {
            Direction::RIGHT => {
                if grid[self.position.0 - 1][self.position.1] == 1 {
                    self.direction = Direction::UP;
                } else {
                    self.direction = Direction::DOWN;
                }
            }
            Direction::DOWN => {
                if grid[self.position.0][self.position.1 - 1] == 1 {
                    self.direction = Direction::LEFT;
                } else {
                    self.direction = Direction::RIGHT;
                }
            }
            Direction::LEFT => {
                if grid[self.position.0 - 1][self.position.1] == 1 {
                    self.direction = Direction::UP;
                } else {
                    self.direction = Direction::DOWN;
                }
            }
            Direction::UP => {
                if grid[self.position.0][self.position.1 - 1] == 1 {
                    self.direction = Direction::LEFT;
                } else {
                    self.direction = Direction::RIGHT;
                }
            }
        }
        self.turns += 1;
    }

    fn next(&self) -> Option<(usize, usize)> {
        let mut position = self.position;
        match self.direction {
            Direction::RIGHT => position.1 += 1,
            Direction::DOWN => position.0 += 1,
            Direction::LEFT => {
                if position.1 == 0 {
                    return None;
                }
                position.1 -= 1
            }
            Direction::UP => {
                if self.position.0 == 0 {
                    return None;
                }
                position.0 -= 1;
            }
        }
        Some(position)
    }

    fn advance(&mut self) -> bool {
        match self.direction {
            Direction::RIGHT => {
                self.position.1 += 1;
                return true;
            }
            Direction::DOWN => {
                self.position.0 += 1;
                return false;
            }
            Direction::LEFT => {
                if self.position.1 == 0 {
                    return false;
                }
                self.position.1 -= 1;
            }
            Direction::UP => {
                if self.position.0 == 0 {
                    return false;
                }
                self.position.0 -= 1;
            }
        }
        true
    }
}

impl PartialEq for Arrow {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.direction == other.direction
    }

    fn ne(&self, other: &Self) -> bool {
        self.position != other.position || self.direction != other.direction
    }
}

/*---------------------------------------------------------------------------*/

fn amount_sides(region: &Vec<Element>) -> u32 {
    let mut grid = [[0; 30]; 30];
    for element in region.iter() {
        let position = ((element.x + 1) * 2, (element.y + 1) * 2);

        let mut up = false;
        let mut right = false;
        let mut left = false;
        let mut down = false;
        for adjacent in element.adjacents.iter() {
            match adjacent {
                Direction::UP => up = true,
                Direction::DOWN => down = true,
                Direction::RIGHT => right = true,
                Direction::LEFT => left = true,
            }
        }
        let value = 1;
        if !up {
            grid[position.0 - 1][position.1] = value;
        }
        if !down {
            grid[position.0 + 1][position.1] = value;
        }
        if !right {
            grid[position.0][position.1 + 1] = value;
        }
        if !left {
            grid[position.0][position.1 - 1] = value;
        }
        if !(up && left) {
            grid[position.0 - 1][position.1 - 1] = value;
        }
        if !(up && right) {
            grid[position.0 - 1][position.1 + 1] = value;
        }
        if !(down && left) {
            grid[position.0 + 1][position.1 - 1] = value;
        }
        if !(down && right) {
            grid[position.0 + 1][position.1 + 1] = value;
        }
    }
    trace!("Grid for {:?}", region);
    for i in 0..30 {
        trace!("{:?}", grid[i]);
    }

    let mut first_position = (0, 0);
    for x in 0..30 {
        for y in 0..30 {
            if grid[x][y] == 1 {
                first_position = (x, y);
                break;
            }
        }
    }
    let first_arrow = Arrow::new(first_position.0, first_position.1);
    let mut arrow = first_arrow.clone();
    let mut next_position: (usize, usize);

    loop {
        trace!(
            "First Arrow: {:?} - Current Arrow: {:?}",
            first_arrow,
            arrow
        );

        match arrow.next() {
            Some(position) => next_position = position,
            None => {
                arrow.turn(&grid);
                continue;
            }
        }

        if grid[next_position.0][next_position.1] == 0 {
            arrow.turn(&grid);
            continue;
        }

        if arrow.turns != 0 && arrow == first_arrow {
            break;
        }
        arrow.advance();
    }

    arrow.turns
}

fn find_region(
    input: &mut Vec<Vec<Plant>>,
    region: &mut Vec<Element>,
    previous_position: (usize, usize),
    current_position: (usize, usize),
) {
    let x = current_position.0;
    let y = current_position.1;
    let mut element = Element {
        x,
        y,
        adjacents: Vec::new(),
        amount_links: 4,
    };

    if input[x][y].visited {
        return;
    }

    input[x][y].visited = true;
    let plant_type = input[x][y].plant_type;

    trace!(
        "Backtracking. Plant type: {} - Prev pos: {:?} - Cur Pos: {:?}",
        plant_type,
        previous_position,
        current_position,
    );

    // TODO: I know these two fors with duplicated code can be made with just one
    // Change it after D12-E2 is solved
    for index in -1..=1 {
        if x as i16 + index < 0 || x as i16 + index >= input.len() as i16 {
            continue;
        }
        let next_x = (x as i16 + index) as usize;

        if index == 0 {
            continue;
        }

        if input[next_x][y].visited {
            if input[next_x][y].plant_type == plant_type {
                if index == -1 {
                    element.adjacents.push(Direction::UP);
                } else {
                    element.adjacents.push(Direction::DOWN);
                }

                element.amount_links -= 1;
            }
            continue;
        }

        if input[next_x][y].plant_type == plant_type {
            if index == -1 {
                element.adjacents.push(Direction::UP);
            } else {
                element.adjacents.push(Direction::DOWN);
            }

            trace!(
                "Match!. Plant type: {} - Cur pos: {:?} - Next Pos: {:?}",
                plant_type,
                current_position,
                (next_x, y)
            );
            element.amount_links -= 1;
            find_region(input, region, current_position, (next_x, y));
        }
    }

    for index in -1..=1 {
        if y as i16 + index < 0 || y as i16 + index >= input.len() as i16 {
            continue;
        }
        let next_y = (y as i16 + index) as usize;

        if index == 0 {
            continue;
        }

        if input[x][next_y].visited {
            if input[x][next_y].plant_type == plant_type {
                if index == -1 {
                    element.adjacents.push(Direction::LEFT);
                } else {
                    element.adjacents.push(Direction::RIGHT);
                }
                element.amount_links -= 1;
            }
            continue;
        }

        if input[x][next_y].plant_type == plant_type {
            if index == -1 {
                element.adjacents.push(Direction::LEFT);
            } else {
                element.adjacents.push(Direction::RIGHT);
            }

            trace!(
                "Match!. Plant type: {} - Cur pos: {:?} - Next Pos: {:?}",
                plant_type,
                current_position,
                (x, next_y)
            );
            element.amount_links -= 1;
            find_region(input, region, current_position, (x, next_y));
        }
    }
    region.push(element);
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut total = 0;
    let mut input: Vec<Vec<Plant>> = Vec::new();
    let mut regions: Vec<(char, Vec<Element>)> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        input.push(
            line.chars()
                .into_iter()
                .map(|x| Plant {
                    plant_type: x,
                    visited: false,
                })
                .collect(),
        );
    }

    // NOTE: Asume the garden is a square
    for x in 0..input.len() {
        for y in 0..input.len() {
            let mut region: Vec<Element> = Vec::new();
            trace!("[Point({},{})] - Value: {:?}", x, y, input[x][y]);
            find_region(&mut input, &mut region, (x, y), (x, y));
            if region.len() > 0 {
                regions.push((input[x][y].plant_type, region));
            }
        }
    }

    debug!("Regions found");
    debug!("{:#?}", regions);

    for region in regions.iter() {
        let area = region.1.len();
        let sides = amount_sides(&region.1);
        total += area * sides as usize;
        info!(
            "Region: {} - Area: {} - Perimeter: {} - Price: {}",
            region.0,
            area,
            sides,
            area * sides as usize,
        );
    }

    info!("Day 12 - Exercise 1. Result: {}", total);
    Ok(())
}

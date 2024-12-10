use log::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

#[derive(Debug, Default, PartialEq, Copy, Clone, Hash, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Default, PartialEq)]
struct Vector {
    a: Point,
    b: Point,
    square_len: i32,
    x_distance: i32,
    y_distance: i32,
}

impl Vector {
    fn new(a: &Point, b: &Point, square_len: i32) -> Vector {
        let x_distance = a.x - b.x;
        let y_distance = a.y - b.y;
        Self {
            a: *a,
            b: *b,
            square_len,
            x_distance,
            y_distance,
        }
    }

    fn calculate_antinode(&self) -> Option<Point> {
        let antinode = Point {
            x: self.b.x - self.x_distance,
            y: self.b.y - self.y_distance,
        };

        if self.point_out_of_bounds(&antinode) {
            return None;
        }
        Some(antinode)
    }

    fn point_out_of_bounds(&self, point: &Point) -> bool {
        if point.x < 0 || point.x > self.square_len - 1 {
            return true;
        }

        if point.y < 0 || point.y > self.square_len - 1 {
            return true;
        }

        return false;
    }
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut antennas_positions: HashMap<char, Vec<Point>> = HashMap::new();

    let mut row = 0;
    for line in reader.lines() {
        let line = line?;
        let mut col = 0;
        for character in line.chars() {
            if character != '.' && !antennas_positions.contains_key(&character) {
                antennas_positions.insert(character, Vec::new());
            }
            antennas_positions.entry(character).and_modify(|entry| {
                let point = Point { x: row, y: col };
                entry.push(point)
            });
            col += 1;
        }
        row += 1;
    }

    // NOTE: Assume the input is a square, meaning that all
    // rows has the same with, same with columns
    let square_len: usize = row.try_into().unwrap();

    // NOTE:
    // * An antinode occurs at any point that is perfectly in line with two antennas of the same frequency, but only when one of the antennas is twice as far away as the other
    // * Antennas with different frequencies do not create antinodes; A and a count as different frequencies
    // * Antinodes that go out of bounds are not counted
    // * Antinodes can occur at locations that contain antennas
    // * How many unique locations within the bounds of the map contain an antinode?
    let mut unique_antinodes: HashSet<Point> = HashSet::new();
    for (key, value) in antennas_positions.iter() {
        trace!("Value: {}", key);
        for first_point in value.iter() {
            trace!("Point: {:#?}", first_point);
            for second_point in value.iter() {
                if first_point == second_point {
                    continue;
                }

                trace!("Evaluating {:?} vs. {:?}", first_point, second_point);
                let vector = Vector::new(first_point, second_point, square_len.try_into().unwrap());
                if let Some(antinode) = vector.calculate_antinode() {
                    unique_antinodes.insert(antinode);
                }
            }
        }
    }

    trace!("Antennas positions");
    trace!("{:?}", antennas_positions);

    info!("Day 8 - Exercise 1. Result: {}", unique_antinodes.len());
    Ok(())
}

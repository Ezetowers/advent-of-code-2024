use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

#[derive(Debug, Default, Copy, Clone)]
struct Plant {
    plant_type: char,
    visited: bool,
}

#[derive(Debug, Copy, Clone)]
struct Element {
    x: usize,
    y: usize,
    amount_links: usize,
}

/*---------------------------------------------------------------------------*/

fn find_region(
    input: &mut Vec<Vec<Plant>>,
    region: &mut Vec<Element>,
    previous_position: (usize, usize),
    current_position: (usize, usize),
) {
    let x = current_position.0;
    let y = current_position.1;
    let mut element = Element {
        x: x,
        y: y,
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

    for index in -1..=1 {
        if x as i8 + index < 0 || x as i8 + index >= input.len() as i8 {
            continue;
        }
        let next_x = (x as i8 + index) as usize;

        if index == 0 {
            continue;
        }

        if input[next_x][y].visited {
            if input[next_x][y].plant_type == plant_type {
                element.amount_links -= 1;
            }
            continue;
        }

        if input[next_x][y].plant_type == plant_type {
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
        if y as i8 + index < 0 || y as i8 + index >= input.len() as i8 {
            continue;
        }
        let next_y = (y as i8 + index) as usize;

        if index == 0 {
            continue;
        }

        if input[x][next_y].visited {
            if input[x][next_y].plant_type == plant_type {
                element.amount_links -= 1;
            }
            continue;
        }

        if input[x][next_y].plant_type == plant_type {
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
        let perimeter = region.1.iter().fold(0, |acc, &x| acc + x.amount_links);
        info!(
            "Region: {} - Area: {} - Perimeter: {}",
            region.0, area, perimeter
        );
        total += area * perimeter;
    }

    info!("Day 12 - Exercise 1. Result: {}", total);
    Ok(())
}

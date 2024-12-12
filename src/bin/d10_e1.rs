use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/
fn trailhike_score(
    input: &Vec<Vec<u8>>,
    point: (i8, i8),
    previous_point: (i8, i8),
    expected_slope: u8,
) -> u32 {
    // NOTE: We asume the map is square
    let map_limit = input.len();
    if point.0 < 0 || point.0 == map_limit as i8 || point.1 < 0 || point.1 == map_limit as i8 {
        return 0;
    }

    let current_slope = input[point.0 as usize][point.1 as usize];

    if current_slope != expected_slope {
        return 0;
    }
    if current_slope == 9 {
        return 1;
    }

    let mut score: u32 = 0;
    for i in -1..=1 {
        if i == 0 {
            continue;
        }
        if previous_point.0 != point.0 || previous_point.1 != point.1 + i {
            trace!(
                "Current: {:?} - Next: {:?} - Current Slope: {:?}",
                point,
                (point.0, point.1 + i),
                current_slope,
            );
            score += trailhike_score(input, (point.0, point.1 + i), point, current_slope + 1);
        }
        if previous_point.0 != point.0 + i || previous_point.1 != point.1 {
            trace!(
                "Current: {:?} - Next: {:?} - Current Slope: {:?}",
                point,
                (point.0 + i, point.1),
                current_slope,
            );
            score += trailhike_score(input, (point.0 + i, point.1), point, current_slope + 1);
        }
    }
    score
}
/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut total = 0;

    let mut input: Vec<Vec<u8>> = Vec::new();
    let mut trailhike_start: Vec<(usize, usize)> = Vec::new();
    let mut x_index = 0;
    for line in reader.lines() {
        let line = line?;
        let mut v: Vec<u8> = Vec::new();
        let mut y_index = 0;
        for character in line.chars() {
            let num = character.to_string().parse::<u8>()?;
            v.push(num);
            if num == 0 {
                trailhike_start.push((x_index, y_index));
            }
            y_index += 1;
        }
        input.push(v);
        x_index += 1;
    }

    // Debugging input
    trace!("Input parsed");
    for line in 0..input.len() {
        trace!("{:?}", input[line]);
    }

    // Trailhike starting points
    trace!("Starting trailhike points");
    for point in trailhike_start.iter() {
        let score = trailhike_score(
            &input,
            (point.0 as i8, point.1 as i8),
            (point.0 as i8, point.1 as i8),
            0,
        );
        trace!("Point: {:?} - Score: {}", point, score);
        total += score;
    }

    info!("Day X - Exercise Y. Result: {}", total);
    Ok(())
}

use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);

    let mut id = 0;
    let mut disk: Vec<String> = Vec::new();
    let mut file_block = true;
    for line in reader.lines() {
        let line = line?;
        for i in line.chars().collect::<Vec<_>>().iter() {
            if file_block {
                for _ in 0..i.to_string().parse::<i32>()? {
                    disk.push(id.to_string());
                }

                file_block = false;
                id += 1;
            } else {
                for _ in 0..i.to_string().parse::<i32>()? {
                    disk.push('.'.to_string());
                }
                file_block = true;
            }
        }
        trace!("Input: {}", line);
    }

    let mut right_index = disk.len() - 1;
    loop {
        while disk[right_index] == "." {
            right_index -= 1;
        }
        trace!("Right Index: {}", right_index);

        let current_id = disk[right_index].clone();
        let right_bound = right_index;
        let mut left_bound = right_index;
        while left_bound != 0 && disk[left_bound] == current_id {
            left_bound -= 1;
        }

        right_index = left_bound;

        trace!("Left bound: {}", left_bound);
        let current_id_empty_space = right_bound - left_bound;
        let mut empty_space = 0;
        for i in 0..disk.len() {
            if disk[i] == "." {
                empty_space += 1;
            } else {
                empty_space = 0;
            }

            if i > left_bound {
                trace!("ID {} cannot be moved, skipping it", current_id);
                break;
            }

            if empty_space == current_id_empty_space {
                debug!("Value to be moved: {} - Current Index: {}", current_id, i);
                for j in 0..current_id_empty_space {
                    disk[i + j - empty_space + 1] = disk[left_bound + 1 + j].clone();
                    disk[left_bound + 1 + j] = ".".to_string();
                }
                break;
            }
        }

        if left_bound == 0 {
            break;
        }
    }

    let mut checksum: u64 = 0;
    debug!("Disk: {:?}", disk);
    for i in 0..disk.len() {
        if disk[i] == "." {
            continue;
        }

        let num = disk[i].parse::<u64>().unwrap_or(0);
        let to_sum = i as u64 * num as u64;
        checksum += to_sum;
        debug!(
            "ID: {} - Number: {} - To sum: {} - Partial checksum: {}",
            i, num, to_sum, checksum
        );
    }

    info!("Day 9 - Exercise 2. Result: {}", checksum);
    Ok(())
}

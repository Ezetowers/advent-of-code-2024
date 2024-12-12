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
        for i in line.chars() {
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
            trace!("Char: {}", i);
        }
        trace!("Input: {}", line);
    }

    let mut still_values_to_move = true;
    let mut left_index = 0;
    let mut right_index = disk.len() - 1;
    while still_values_to_move {
        while disk[left_index] != "." {
            left_index += 1;
        }

        while disk[right_index] == "." {
            right_index -= 1;
        }

        if left_index >= right_index {
            still_values_to_move = false;
            continue;
        }

        let aux = disk[right_index].clone();
        disk[right_index] = disk[left_index].clone();
        disk[left_index] = aux;
    }

    let mut checksum: u64 = 0;
    for i in 0..disk.len() {
        if disk[i] == "." {
            break;
        }

        let num = disk[i].parse::<u64>().unwrap_or(0);
        trace!(
            "ID: {} - Number: {} - Partial checksum: {}",
            i,
            num,
            checksum
        );
        checksum += i as u64 * num as u64;
    }

    info!("Day 9 - Exercise 1. Result: {}", checksum);
    Ok(())
}

use log::*;
use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

fn blink(stone: u64) -> Vec<u64> {
    let mut after_blink: Vec<u64> = Vec::new();
    // Rule 1: If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
    if stone == 0 {
        after_blink.push(1);
        return after_blink;
    }

    // Rule 2: If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
    let string_number = stone.to_string();
    if string_number.len() % 2 == 0 {
        let first_number_string = &string_number[0..=string_number.len() / 2 - 1];
        let first_number = first_number_string.to_string().parse::<u64>().unwrap_or(0);
        let second_number_string =
            &string_number[string_number.len() / 2..=string_number.len() - 1];
        let second_number = second_number_string.to_string().parse::<u64>().unwrap_or(0);
        trace!(
                "Case 2: First number string: {} - First number parsed: {} - Second number string: {} - Second number parsed: {}",
                first_number_string,
                first_number,
                second_number_string,
                second_number,
            );
        after_blink.push(first_number);
        after_blink.push(second_number);
        return after_blink;
    }

    // Default: If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
    after_blink.push(stone * 2024 as u64);
    after_blink
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut input: Vec<u64> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        input = line
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap_or(0))
            .collect();

        // NOTE: The input only has line
        break;
    }

    // Input debugging
    trace!("Input: {:?}", input);
    let mut previous_stones: HashMap<u64, u64> = HashMap::new();
    for i in 0..input.len() {
        previous_stones
            .entry(input[i])
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for i in 1..=75 {
        let mut current_stones: HashMap<u64, u64> = HashMap::new();
        for (k, v) in previous_stones.iter() {
            let stones = blink(*k);
            for stone in 0..stones.len() {
                current_stones
                    .entry(stones[stone])
                    .and_modify(|count| *count += v)
                    .or_insert(*v);
            }
        }
        previous_stones = current_stones;

        let mut partial_stones = 0;
        for (_, v) in previous_stones.iter() {
            partial_stones += v;
        }
        debug!("[BLINK {}] Partial Stones: {:?}", i, partial_stones);
    }

    let mut total_stones = 0;
    for (_k, v) in previous_stones.iter() {
        total_stones += v;
    }

    info!("Day 11 - Exercise 1. Result: {}", total_stones);
    Ok(())
}

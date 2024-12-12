use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

fn blink(input: &Vec<u64>) -> Vec<u64> {
    let mut after_blink: Vec<u64> = Vec::new();
    for number in input.iter() {
        // Rule 1: If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
        if *number == 0 {
            after_blink.push(1);
            continue;
        }

        // Rule 2: If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
        let string_number = number.to_string();
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
            continue;
        }

        // Default: If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
        after_blink.push(number * 2024 as u64);
    }

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
    for i in 1..=25 {
        input = blink(&input);
        debug!("[BLINK {}] output: {:?}", i, input);
    }
    info!("Day 11 - Exercise 1. Result: {}", input.len());
    Ok(())
}

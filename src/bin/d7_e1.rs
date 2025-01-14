use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);

    let mut total: u64 = 0;
    let mut line_number = 0;

    for line in reader.lines() {
        let line = line?;
        let split_line: Vec<&str> = line.split(":").collect();
        let test_result = split_line[0].parse::<u64>().unwrap_or(0);
        let test_values: Vec<u64> = split_line[1]
            .trim()
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap_or(0))
            .collect();

        let permutations = usize::pow(2, (test_values.len() - 1).try_into().unwrap());
        trace!(
            "Test Result: {} - Test Values: {:?} - Permutations: {}",
            test_result,
            test_values,
            permutations,
        );

        let mut valid_line = false;
        // NOTE: Since we only have two operations, the amount of permutations that we can have
        // can be thought as the a decomposition of a 2 power number, where every bit defines
        // which operation needs to be made
        // E.g. if we have 3 test_values and two operations (+,*), then there are 2^(len-1) permutations
        // that can be made
        // Permutations:
        //  * 0: 0 0
        //  * 1: 0 1
        //  * 2: 1 0
        //  * 3: 1 1
        // If we replace 0 with + and 1 with *, then you have the problem solved
        for permutation in 0..permutations {
            let mut permutation_result: u64 = test_values[0];
            for j in 0..test_values.len() - 1 {
                let operation = permutation / usize::pow(2, j.try_into().unwrap()) % 2;
                if operation == 1 {
                    permutation_result = permutation_result * test_values[j + 1];
                } else {
                    permutation_result = permutation_result + test_values[j + 1];
                }
                trace!(
                    "Permutation: {} - Operation: {} - Partial result: {}",
                    permutation,
                    operation,
                    permutation_result
                );
            }

            if test_result == permutation_result {
                total += permutation_result;
                valid_line = true;
                break;
            }
        }
        debug!(
            "Line number: {} - Valid line: {} - Partial total: {} - Line: {}",
            line_number, valid_line, total, line
        );
        line_number += 1;
    }

    info!("Day 7 - Exercise 1. Result: {}", total);
    Ok(())
}

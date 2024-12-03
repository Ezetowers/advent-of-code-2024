use log2::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let mul_operation = Vec::from(['m', 'u', 'l', '(']);
    let _log2 = log2::stdout().module(false).level("info").start();

    let file = File::open("./input/d3.txt")?;
    let reader = BufReader::new(file);
    let mut total = 0;

    for line in reader.lines() {
        let line = line?;
        let my_chars: Vec<_> = line.chars().collect();

        let mut curr_pointer = 0;
        let mut offset = 0;

        let mut mul_found = false;
        let mut first_number_found = false;
        let mut first_number = 0;
        let mut second_number_found = false;
        let mut second_number = 0;

        for i in 0..my_chars.len() {
            if mul_found && first_number_found && second_number_found {
                total += first_number * second_number;
                debug!(
                    "Bingo. mul({},{}) - Mutiplication: {} - Total: {}",
                    first_number,
                    second_number,
                    first_number * second_number,
                    total
                );
                mul_found = false;
                first_number_found = false;
                second_number_found = false;
            }

            trace!(
                "Index: {:#?} - Char: {:#?} - Cur Pointer: {:#?} - Offset: {:#?}",
                i,
                my_chars[i],
                curr_pointer,
                offset
            );

            // Check if the string starts with mul(
            if !mul_found {
                if my_chars[curr_pointer + offset] == mul_operation[offset] {
                    if offset == 3 {
                        mul_found = true;
                        trace!("Mul found!");
                        curr_pointer += offset + 1;
                        offset = 0;
                    } else {
                        offset += 1;
                    }
                } else {
                    // mul not found, advance pointers and start all over again
                    // Only offset needs to be reset
                    curr_pointer += offset + 1;
                    offset = 0;
                }
                continue;
            }

            if !first_number_found {
                let is_digit = my_chars[curr_pointer + offset].is_digit(10);
                let is_delimiter = my_chars[curr_pointer + offset] == ',';

                if !is_digit && !is_delimiter {
                    curr_pointer += offset + 1;
                    offset = 0;
                    mul_found = false;
                    continue;
                }

                if is_delimiter {
                    trace!("First number - Delimiter found");
                    // Verify if there is no number to be parsed [mul(,123)]
                    // case
                    if offset == 0 {
                        trace!("Number not found in mul directive. Resetting mul parsing");
                        curr_pointer += 1;
                        mul_found = false;
                        continue;
                    }

                    let number_vec = &my_chars[curr_pointer..curr_pointer + offset].to_vec();
                    let number_string: String = number_vec.into_iter().collect();
                    first_number = number_string.parse::<i32>().unwrap_or(0);
                    trace!("First number parsed: {}", first_number);

                    // Verify if the number processed has more than 3 digits
                    if first_number > 999 {
                        trace!("Number is bigger than expected. Resetting mul parsing");
                        curr_pointer += 1;
                        mul_found = false;
                        continue;
                    }

                    first_number_found = true;
                    curr_pointer += offset + 1;
                    offset = 0;
                    continue;
                }

                if is_digit {
                    offset += 1;
                    continue;
                }
            }

            if !second_number_found {
                let is_digit = my_chars[curr_pointer + offset].is_digit(10);
                let is_delimiter = my_chars[curr_pointer + offset] == ')';

                if !is_digit && !is_delimiter {
                    curr_pointer += offset + 1;
                    offset = 0;
                    mul_found = false;
                    first_number_found = false;
                    continue;
                }

                if is_delimiter {
                    trace!("Second number - Delimiter found");
                    // Verify if there is no number to be parsed [mul(,123)]
                    // case
                    if offset == 0 {
                        trace!("Number not found in mul directive. Resetting mul parsing");
                        curr_pointer += 1;
                        mul_found = false;
                        first_number_found = false;
                        continue;
                    }

                    let number_vec = &my_chars[curr_pointer..curr_pointer + offset].to_vec();
                    let number_string: String = number_vec.into_iter().collect();
                    second_number = number_string.parse::<i32>().unwrap_or(0);
                    trace!("Second number parsed: {}", second_number);

                    // Verify if the number processed has more than 3 digits
                    if second_number > 999 {
                        trace!("Number is bigger than expected. Resetting mul parsing");
                        curr_pointer += 1;
                        mul_found = false;
                        first_number_found = false;
                        continue;
                    }

                    second_number_found = true;
                    curr_pointer += offset + 1;
                    offset = 0;
                    continue;
                }

                if is_digit {
                    offset += 1;
                    continue;
                }
            }
        }

        if mul_found && first_number_found && second_number_found {
            total += first_number * second_number;
            debug!(
                "Bingo. mul({},{}) - Mutiplication: {} - Total: {}",
                first_number,
                second_number,
                first_number * second_number,
                total
            );
        }
    }

    info!("[Day 3 - Exercise 1] Result: {}", total);
    Ok(())
}

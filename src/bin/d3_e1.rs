use log::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn setup_logger() {
    pretty_env_logger::init_timed();
}

fn main() -> Result<(), Box<dyn Error>> {
    let mul_operation = Vec::from(['m', 'u', 'l', '(']);
    setup_logger();

    let file = File::open("./input/d3-minimal.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let my_chars: Vec<_> = line.chars().collect();

        let line_length = my_chars.len();
        let mut curr_pointer = 0;

        let mut mul_found = false;
        let mut offset = 0;

        let mut first_number = 0;
        let mut first_number_found = false;

        let mut second_number_found = false;

        for i in 0..my_chars.len() {
            debug!(
                "Index: {:#?} - Char: {:#?} - Cur Pointer: {:#?} - Offset: {:#?}",
                i, my_chars[i], curr_pointer, offset
            );

            // Check if the string starts with mul(
            if !mul_found {
                if my_chars[curr_pointer + offset] == mul_operation[offset] {
                    if offset == 3 {
                        mul_found = true;
                        debug!("Mul found!");
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
                    mul_found = false;
                    curr_pointer += offset + 1;
                    offset = 0;
                    continue;
                }

                if is_delimiter {
                    debug!("Delimiter found");
                    // TODO: Verify if there is a number to be parsed [mul(,123)]
                    // case
                    // TODO: Verify if the number processed has more than 3 digits
                    let number_vec = &my_chars[curr_pointer..curr_pointer + offset].to_vec();
                    let number_string: String = number_vec.into_iter().collect();
                    let number = number_string.parse::<i32>().unwrap_or(0);
                    debug!("Number parsed: {}", number);
                    first_number_found = true;
                    curr_pointer += offset + 1;
                    offset += 0;
                    continue;
                }

                if is_digit {
                    offset += 1;
                    continue;
                }
            }
        }
    }

    info!("[Day 2 - Exercise 2] Result: NOPE");
    Ok(())
}

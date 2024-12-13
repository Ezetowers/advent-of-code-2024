use log::*;
use std::collections::VecDeque;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::thread;
use std::thread::JoinHandle;

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

#[derive(Debug, Default, Copy, Clone)]
struct Stone {
    value: u64,
    blink: usize,
}

/*---------------------------------------------------------------------------*/

fn blink(stone: Stone) -> Vec<Stone> {
    let mut after_blink: Vec<Stone> = Vec::new();
    // Rule 1: If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
    if stone.value == 0 {
        after_blink.push(Stone {
            value: 1,
            blink: stone.blink + 1,
        });
        return after_blink;
    }

    // Rule 2: If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
    let string_number = stone.value.to_string();
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
        after_blink.push(Stone {
            value: first_number,
            blink: stone.blink + 1,
        });
        after_blink.push(Stone {
            value: second_number,
            blink: stone.blink + 1,
        });
        return after_blink;
    }

    // Default: If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
    after_blink.push(Stone {
        value: stone.value * 2024 as u64,
        blink: stone.blink + 1,
    });

    after_blink
}

/*---------------------------------------------------------------------------*/

fn process_stone(stone: Stone) -> u64 {
    let mut stones: VecDeque<Stone> = VecDeque::new();
    stones.push_back(stone);

    let mut total = 0;
    while stones.len() > 0 {
        let stone = stones.pop_front().unwrap();
        if stone.blink != 75 {
            for new_stone in blink(stone) {
                stones.push_front(new_stone);
            }
        } else {
            total += 1;
        }
    }
    total
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

    // Convert the input in stones
    let mut handles: Vec<JoinHandle<u64>> = Vec::new();
    for i in 0..input.len() {
        let stone_number = input[i];
        let handle = thread::spawn(move || {
            let stone = Stone {
                value: stone_number,
                blink: 0,
            };
            info!("[Thread {}] Processing initial stone {:#?}", i, stone);
            process_stone(stone)
        });
        handles.push(handle);
    }

    let mut total = 0;
    let mut counter = 0;
    for handle in handles.into_iter() {
        info!("Waiting thread {} to finish", counter);
        match handle.join() {
            Ok(subtotal) => {
                total += subtotal;
                info!(
                    "Thread {} finished. Subtotal: {} - Total: {}",
                    counter, subtotal, total
                );
            }
            Err(e) => panic!("{:#?}", e),
        }
        counter += 1;
    }

    info!("Day 11 - Exercise 2. Result: {}", total);
    Ok(())
}

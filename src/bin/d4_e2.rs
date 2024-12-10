use log2::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*---------------------------------------------------------------------------*/

fn setup_logger() -> log2::Handle {
    let log_level = match std::env::var("LOG_LEVEL") {
        Ok(val) => val,
        Err(_) => "info".to_string(),
    };
    log2::stdout().module(false).level(log_level).start()
}

fn setup_input() -> std::io::Result<File> {
    let input_path = match std::env::var("INPUT_PATH") {
        Ok(val) => val,
        Err(_) => panic!("Invalid INPUT_PATH. Check if path exists"),
    };
    File::open(&input_path)
}

/*---------------------------------------------------------------------------*/

fn count_matches(input: &Vec<Vec<char>>, x: usize, y: usize, string_to_match: &String) -> i32 {
    let mut first_x = String::new();
    let mut second_x = String::new();
    for i in 0..3 {
        first_x.push(input[x + i][y + i]);
        second_x.push(input[x + i][y + 2 - i]);
    }
    let first_x_inverted = first_x.chars().rev().collect::<String>();
    let second_x_inverted = second_x.chars().rev().collect::<String>();
    let bingo = (first_x == *string_to_match || first_x_inverted == *string_to_match)
        && (second_x == *string_to_match || second_x_inverted == *string_to_match);

    trace!(
        "First X: {} - First X inverted: {} - Second X: {} - Second X inverted: {} - Match: {}",
        first_x,
        first_x_inverted,
        second_x,
        second_x_inverted,
        bingo
    );

    if !first_x.contains(string_to_match)
        && !first_x
            .chars()
            .rev()
            .collect::<String>()
            .contains(string_to_match)
    {
        return 0;
    }

    if !second_x.contains(string_to_match)
        && !second_x
            .chars()
            .rev()
            .collect::<String>()
            .contains(string_to_match)
    {
        return 0;
    }

    1
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = setup_logger();
    let reader = BufReader::new(setup_input()?);

    let keyword = String::from("MAS");
    let mut input: Vec<Vec<char>> = Vec::new();
    let mut total = 0;

    for line in reader.lines() {
        let line = line?;
        let my_chars: Vec<_> = line.chars().collect();
        input.push(my_chars);
    }

    // NOTE: Assume the input is a square, meaning that all
    // rows has the same with, same with columns
    let square_len: usize = input.len();

    for x in 0..square_len - 2 {
        for y in 0..square_len - 2 {
            trace!("Position ({},{})", x, y);
            total += count_matches(&input, x, y, &keyword);
        }
    }

    info!("[Day 4 - Exercise 2] Result: {}", total);
    Ok(())
}

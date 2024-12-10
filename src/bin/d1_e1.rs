use log::*;
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

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = setup_logger();
    let reader = BufReader::new(setup_input()?);

    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let v: Vec<i32> = line
            .split("   ")
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .collect();
        first_list.push(v[0]);
        second_list.push(v[1]);
    }

    first_list.sort();
    second_list.sort();

    let n: usize = first_list.len();
    let mut sum = 0;
    for i in 0..n {
        let difference = first_list[i] - second_list[i];
        sum += difference.abs();
    }
    info!("Day 1 - Exercise 1 result: {}", sum);
    Ok(())
}

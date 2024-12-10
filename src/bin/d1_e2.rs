use log::*;
use std::collections::HashMap;
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
    let mut second_list_hash: HashMap<i32, i32> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let v: Vec<i32> = line
            .split("   ")
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .collect();
        first_list.push(v[0]);
        second_list_hash
            .entry(v[1])
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let n: usize = first_list.len();
    let mut score = 0;
    for i in 0..n {
        score += first_list[i] * *second_list_hash.entry(first_list[i]).or_default();
    }
    info!("Day 1 - Exercise 2 result: {}", score);
    Ok(())
}

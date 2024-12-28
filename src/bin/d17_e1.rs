use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

struct Computer {
    reg_a: i32,
    reg_b: i32,
    reg_c: i32,
}

impl Computer {
    fn new(reg_a: i32, reg_b: i32, reg_c: i32) -> Computer {
        Self {
            reg_a,
            reg_b,
            reg_c,
        }
    }

    fn execute_program(&mut self, program: &Vec<i32>) -> Vec<i32> {
        let result: Vec<i32> = Vec::new();
        result
    }
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);

    // File format
    //   Register A: <number>
    //   Register B: <number>
    //   Register C: <number>
    //
    //   Program: <number (between 0..7)>, ..., <number (between 0..7)>
    let mut lines_it = reader.lines();
    let reg_a_string = lines_it.next().unwrap()?.split(':').collect::<Vec<_>>()[1].to_string();
    let reg_a = reg_a_string.trim().parse::<i32>().unwrap_or(0);
    trace!("Reg A. String: {} - Number: {}", reg_a_string, reg_a);

    let reg_b_string = lines_it.next().unwrap()?.split(':').collect::<Vec<_>>()[1].to_string();
    let reg_b = reg_b_string.trim().parse::<i32>().unwrap_or(0);
    trace!("Reg B. String: {} - Number: {}", reg_b_string, reg_b);

    let reg_c_string = lines_it.next().unwrap()?.split(':').collect::<Vec<_>>()[1].to_string();
    let reg_c = reg_c_string.trim().parse::<i32>().unwrap_or(0);
    trace!("Reg C. String: {} - Number: {}", reg_c_string, reg_c);

    lines_it.next();

    let program_string = lines_it.next().unwrap()?.split(':').collect::<Vec<_>>()[1].to_string();
    let program: Vec<i32> = program_string
        .split(',')
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .collect();

    let mut computer = Computer::new(reg_a, reg_b, reg_c);
    let result = computer.execute_program(&program);

    trace!("Program. String: {} - Array: {:?}", program_string, program);

    info!("Day X - Exercise Y. Result: {:?}", result);
    Ok(())
}

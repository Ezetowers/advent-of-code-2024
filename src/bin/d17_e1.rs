use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

#[derive(Debug)]
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
        let mut output: Vec<i32> = Vec::new();

        let mut ip = 0;
        loop {
            // NOTE: Assume we get an even number of elements in the program
            if program.len() == ip {
                break;
            }
            let opcode = program[ip];
            let operand = program[ip + 1];

            debug!("Opcode: {} - Operand: {}", opcode, operand);
            match opcode {
                0 => {
                    self.reg_a = self.reg_a / i32::pow(2, self.combo_operand(operand) as u32);
                }
                1 => {
                    self.reg_b = self.reg_b ^ operand;
                }
                2 => {
                    self.reg_b = self.combo_operand(operand) % 8;
                }
                3 => {
                    if self.reg_a != 0 {
                        ip = operand as usize;
                    }
                }
                4 => {
                    self.reg_b = self.reg_b ^ self.reg_c;
                }
                5 => {
                    output.push(self.combo_operand(operand) % 8);
                }
                6 => {
                    self.reg_b = self.reg_a / i32::pow(2, self.combo_operand(operand) as u32);
                }
                7 => {
                    self.reg_c = self.reg_a / i32::pow(2, self.combo_operand(operand) as u32);
                }
                _ => panic!("This should not happen"),
            }

            if opcode == 3 && self.reg_a != 0 {
                continue;
            } else {
                ip += 2;
            }
            debug!("Computer: {:?} - Current output: {:?}", self, output);
        }
        output
    }

    fn combo_operand(&self, operand: i32) -> i32 {
        match operand {
            0 | 1 | 2 | 3 => operand,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("This should not happen"),
            _ => panic!("This should not happen"),
        }
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
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .collect();

    let mut computer = Computer::new(reg_a, reg_b, reg_c);
    trace!("Program. String: {} - Array: {:?}", program_string, program);

    let result = computer.execute_program(&program);
    info!(
        "Day X - Exercise Y. Computer: {:#?} - Result: {:?}",
        computer, result
    );
    Ok(())
}

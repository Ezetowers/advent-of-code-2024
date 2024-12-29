use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

#[derive(Debug)]
struct Computer {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
}

impl Computer {
    fn new(reg_a: u64, reg_b: u64, reg_c: u64) -> Computer {
        Self {
            reg_a,
            reg_b,
            reg_c,
        }
    }

    fn execute_program(&mut self, program: &Vec<u64>) -> u64 {
        let mut self_found = false;
        let base = 11784601076;
        let mut counter = 0;
        while !self_found {
            let reg_a = base + counter * 12750684160 / 16;
            if reg_a % 10000000 == 0 {
                info!("Current Iteration: {}", reg_a);
            }
            let mut output: Vec<u64> = Vec::new();
            let mut ip = 0;
            self.reg_a = reg_a;
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
                        self.reg_a = self.reg_a / u64::pow(2, self.combo_operand(operand) as u32);
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
                        if output.len() <= program.len() {
                            if output[output.len() - 1] != program[output.len() - 1] {
                                output.pop();
                                break;
                            }
                        }
                    }
                    6 => {
                        self.reg_b = self.reg_a / u64::pow(2, self.combo_operand(operand) as u32);
                    }
                    7 => {
                        self.reg_c = self.reg_a / u64::pow(2, self.combo_operand(operand) as u32);
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

            // Sanity check
            debug!("Computer: {:?} - Current output: {:?}", self, output);
            if output.len() > 12 {
                info!(
                    "Watch out!! - Current Iteration: {} - Computer: {:?} - Output: {:?}",
                    reg_a, self, output,
                );
            }

            if output.len() != program.len() {
                continue;
            }

            for i in 0..output.len() {
                if output[i] != program[i] {
                    self_found = false;
                    break;
                }
            }
            counter += 1;
        }
        base + counter * 12750684160
    }

    fn combo_operand(&self, operand: u64) -> u64 {
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
    let reg_a = reg_a_string.trim().parse::<u64>().unwrap_or(0);
    trace!("Reg A. String: {} - Number: {}", reg_a_string, reg_a);

    let reg_b_string = lines_it.next().unwrap()?.split(':').collect::<Vec<_>>()[1].to_string();
    let reg_b = reg_b_string.trim().parse::<u64>().unwrap_or(0);
    trace!("Reg B. String: {} - Number: {}", reg_b_string, reg_b);

    let reg_c_string = lines_it.next().unwrap()?.split(':').collect::<Vec<_>>()[1].to_string();
    let reg_c = reg_c_string.trim().parse::<u64>().unwrap_or(0);
    trace!("Reg C. String: {} - Number: {}", reg_c_string, reg_c);

    lines_it.next();

    let program_string = lines_it.next().unwrap()?.split(':').collect::<Vec<_>>()[1].to_string();
    let program: Vec<u64> = program_string
        .trim()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap_or(0))
        .collect();

    let mut computer = Computer::new(reg_a, reg_b, reg_c);
    trace!("Program. String: {} - Array: {:?}", program_string, program);

    let result = computer.execute_program(&program);
    info!(
        "Day X - Exercise Y. Computer: {:?} - Result: {}",
        computer, result
    );
    Ok(())
}

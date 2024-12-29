use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::thread;

use advent_of_code_2024::common;

#[derive(Debug)]
struct Computer {
    thread_num: u64,
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
}

impl Computer {
    fn new(thread_num: u64, reg_a: u64, reg_b: u64, reg_c: u64) -> Computer {
        Self {
            thread_num,
            reg_a,
            reg_b,
            reg_c,
        }
    }

    fn execute_program(&mut self, program: &Vec<u64>, base: u64, multiplier: u64) -> u64 {
        let mut self_found = false;
        let mut counter = 0;
        while !self_found {
            let reg_a = base + counter * multiplier;
            if reg_a % 10000000 == 0 {
                debug!("Current Iteration: {}", reg_a);
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
            if output.len() > 10 {
                info!(
                    "[Thread num {}] Watch out!! - Current Iteration: {} - Output: {:?} - Len: {} - {:?}",
                    self.thread_num,
                    reg_a,
                    output,
                    output.len(),
                    self,
                );
            }

            self_found = output.len() == program.len();
            if self_found {
                for i in 0..output.len() {
                    if output[i] != program[i] {
                        self_found = false;
                        break;
                    }
                }
            }
            counter += 1;
        }
        base + counter * multiplier
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

    // let program_string = lines_it.next().unwrap()?.split(':').collect::<Vec<_>>()[1].to_string();
    let max_number: u64 = 69225300431614;
    // thread::scope(|s| {
    let mut join_handlers = Vec::new();
    for i in 0..10 {
        let handle = move |x: u64, some_program_string: &str| {
            let base = max_number / 10 * x;
            let program: Vec<u64> = some_program_string
                .trim()
                .split(',')
                .map(|x| x.parse::<u64>().unwrap_or(0))
                .collect();
            let mut computer = Computer::new(i, reg_a, reg_b, reg_c);
            computer.execute_program(&program, base, 1)
        };

        join_handlers.push(thread::spawn(move || {
            handle(i, "2,4,1,2,7,5,1,3,4,4,5,5,0,3,3,0")
        }));
    }

    for handler in join_handlers.into_iter() {
        handler.join().unwrap();
    }
    // });

    // let handlers = Vec::new();
    // for i in 0..10 {
    //     handlers.push(thread::spawn(move || {
    //         let mut computer = Computer::new(reg_a, reg_b, reg_c);
    //         trace!("Program. String: {} - Array: {:?}", program_string, program);
    //         computer.execute_program(&program, 0, 1)
    //     });
    // }

    // let result = computer.execute_program(&program, 0, 1);
    // info!("Day 17 - Exercise 2. {:?} - Result: {}", computer, result);
    Ok(())
}

use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

const BUTTON_A_TOKENS: u64 = 3;
const BUTTON_B_TOKENS: u64 = 1;
const PRIZE_CONSTANT: u64 = 10000000000000;

/*---------------------------------------------------------------------------*/

#[derive(Debug, Default, Copy, Clone)]
struct Button {
    x: u64,
    y: u64,
    tokens: u64,
}

#[derive(Debug, Default, Copy, Clone)]
struct ClawMachine {
    button_a: Button,
    button_b: Button,
    prize: (u64, u64),
}

/*---------------------------------------------------------------------------*/
fn solve_equation(claw_machine: ClawMachine) -> u64 {
    let prize_x = claw_machine.prize.0 as f64;
    let prize_y = claw_machine.prize.1 as f64;
    let a_x = claw_machine.button_a.x as f64;
    let a_y = claw_machine.button_a.y as f64;
    let b_x = claw_machine.button_b.x as f64;
    let b_y = claw_machine.button_b.y as f64;

    let b = (prize_y - (a_y * prize_x / a_x)) / (b_y - (a_y * b_x) / a_x);
    let a = (prize_x - b_x * b) / a_x;
    debug!("{:?} - A: {} - B: {}", claw_machine, a, b);

    if a < 0.0 || b < 0.0 {
        return 0;
    }

    // Float precision is messing or calculations. Round the values obtained and
    // validate if they match the prizes
    let int_a = a.round() as u64;
    let int_b = b.round() as u64;
    let obtained_prize_x = int_a * a_x as u64 + int_b * b_x as u64;
    let obtained_prize_y = int_a * a_y as u64 + int_b * b_y as u64;
    debug!(
        "{:?} - Int A: {} - B: {} - Obtained Prize X: {} - Obtained Prize Y: {}",
        claw_machine, int_a, int_b, obtained_prize_x, obtained_prize_y
    );
    if obtained_prize_x != prize_x as u64 || obtained_prize_y != prize_y as u64 {
        return 0;
    }

    int_a * claw_machine.button_a.tokens + int_b * claw_machine.button_b.tokens
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut total = 0;

    let mut index = 0;
    let mut claw_machines: Vec<ClawMachine> = Vec::new();
    let mut claw_machine: ClawMachine = Default::default();
    for line in reader.lines() {
        let line = line?;
        if line.len() == 0 {
            index = 0;
            claw_machines.push(claw_machine);
            continue;
        }

        // This line parses this kind of input
        // Button A: X+94, Y+34
        // The result is the vec: [94,34]
        // Or Prize: X=8400, Y=5400
        // The result is the vec: [8400, 5400]
        let delimiter = if index != 2 { "+" } else { "=" };
        let v: Vec<u64> = line.split(":").collect::<Vec<_>>()[1]
            .split(",")
            .map(|x| {
                trace!("Element: {}", x);
                x.split(delimiter).collect::<Vec<_>>()[1]
                    .parse::<u64>()
                    .unwrap_or(0)
            })
            .collect();

        if index == 0 {
            // Button A
            claw_machine.button_a = Button {
                x: v[0],
                y: v[1],
                tokens: BUTTON_A_TOKENS,
            }
        } else if index == 1 {
            // Button B
            claw_machine.button_b = Button {
                x: v[0],
                y: v[1],
                tokens: BUTTON_B_TOKENS,
            };
        } else {
            // Prize
            claw_machine.prize = (v[0] + PRIZE_CONSTANT, v[1] + PRIZE_CONSTANT);
        }
        index += 1;
    }
    claw_machines.push(claw_machine);

    for i in 0..claw_machines.len() {
        let tokens = solve_equation(claw_machines[i]);
        total += tokens;
        debug!(
            "[INDEX {}] {:?} - Solution tokens: {} - Partial total: {}",
            i, claw_machines[i], tokens, total
        );
    }

    info!("Day 13 - Exercise 1. Result: {}", total);
    Ok(())
}

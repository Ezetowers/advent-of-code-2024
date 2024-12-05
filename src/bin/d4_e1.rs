use log2::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let xmas = "XMAS";

    let _log2 = log2::stdout().module(false).level("info").start();
    let file = File::open("./input/d4.txt")?;
    let reader = BufReader::new(file);
    let mut input: Vec<Vec<char>> = Vec::new();
    let mut total = 0;

    for line in reader.lines() {
        let line = line?;
        let my_chars: Vec<_> = line.chars().collect();
        input.push(my_chars);

        // Optimization: Let's do the right and left cases here
        trace!("[LEFT - RIGHT] Line: {}", line);
        total += line.match_indices(xmas).collect::<Vec<_>>().len();
        trace!("[LEFT - RIGHT] Total after processing line: {}", total);
        trace!(
            "[LEFT - RIGHT] Line reverted: {}",
            line.chars().rev().collect::<String>()
        );
        total += line
            .chars()
            .rev()
            .collect::<String>()
            .match_indices(xmas)
            .collect::<Vec<_>>()
            .len();
        trace!(
            "[LEFT - RIGHT] Total after processing line reverted: {}",
            total
        );
    }

    // NOTE: Assume the input is a square, meaning that all
    // rows has the same with, same with columns
    let square_len: usize = input.len();

    // We have already calculated the matches in right, left direction. We know need to do that in:
    // * up and down
    // * diagonals

    // Up and down
    for i in 0..square_len {
        let mut line = String::new();
        for j in 0..square_len {
            line.push(input[j][i]);
        }
        trace!("[UP - DOWN] Line: {}", line);
        total += line.match_indices(xmas).collect::<Vec<_>>().len();
        trace!("[UP - DOWN] Total after processing line: {}", total);
        trace!(
            "[UP - DOWN] Line reverted: {}",
            line.chars().rev().collect::<String>()
        );
        total += line
            .chars()
            .rev()
            .collect::<String>()
            .match_indices(xmas)
            .collect::<Vec<_>>()
            .len();
        trace!(
            "[UP - DOWN] Total after processing line reverted: {}",
            total
        );
    }

    // Left Diagonal
    for i in 0..square_len {
        let mut left_line = String::new();
        let mut right_line = String::new();
        for j in i..square_len {
            left_line.push(input[j][j - i]);
            right_line.push(input[j - i][j]);
        }

        trace!("[LEFT DIAGONAL] Line: {}", left_line);
        total += left_line.match_indices(xmas).collect::<Vec<_>>().len();
        trace!("[LEFT DIAGONAL] Total after processing line: {}", total);
        trace!(
            "[LEFT_DIAGONAL] Line reverted: {}",
            left_line.chars().rev().collect::<String>()
        );
        total += left_line
            .chars()
            .rev()
            .collect::<String>()
            .match_indices(xmas)
            .collect::<Vec<_>>()
            .len();
        trace!(
            "[LEFT DIAGONAL] Total after processing line reverted: {}",
            total
        );
        if i != 0 {
            // Optimization: Let's do the right and left cases here
            trace!("[LEFT_DIAGONAL] Line: {}", right_line);
            total += right_line.match_indices(xmas).collect::<Vec<_>>().len();
            trace!("[LEFT_DIAGONAL] Total after processing line: {}", total);
            trace!(
                "[LEFT_DIAGONAL] Line reverted: {}",
                right_line.chars().rev().collect::<String>()
            );
            total += right_line
                .chars()
                .rev()
                .collect::<String>()
                .match_indices(xmas)
                .collect::<Vec<_>>()
                .len();
            trace!(
                "[LEFT DIAGONAL] Total after processing line reverted: {}",
                total
            );
        }
    }

    // Right diagonal
    for i in 0..square_len {
        let mut left_line = String::new();
        let mut right_line = String::new();
        for j in 0..square_len - i {
            left_line.push(input[square_len - 1 - j - i][j]);
            right_line.push(input[square_len - 1 - j][j + i]);
        }

        // Optimization: Let's do the right and left cases here
        trace!("[RIGHT DIAGONAL] Line: {}", left_line);
        total += left_line.match_indices(xmas).collect::<Vec<_>>().len();
        trace!("[RIGHT DIAGONAL] Total after processing line: {}", total);
        trace!(
            "[RIGHT DIAGONAL] Line reverted: {}",
            left_line.chars().rev().collect::<String>()
        );
        total += left_line
            .chars()
            .rev()
            .collect::<String>()
            .match_indices(xmas)
            .collect::<Vec<_>>()
            .len();
        trace!(
            "[RIGHT DIAGONAL] Total after processing line reverted: {}",
            total
        );
        if i != 0 {
            // Optimization: Let's do the right and left cases here
            trace!("[RIGHT DIAGONAL] Line: {}", right_line);
            total += right_line.match_indices(xmas).collect::<Vec<_>>().len();
            trace!("[RIGHT DIAGONAL] Total after processing line: {}", total);
            trace!(
                "[RIGHT DIAGONAL] Line reverted: {}",
                right_line.chars().rev().collect::<String>()
            );
            total += right_line
                .chars()
                .rev()
                .collect::<String>()
                .match_indices(xmas)
                .collect::<Vec<_>>()
                .len();
            trace!(
                "[RIGHT DIAGONAL] Total after processing line reverted: {}",
                total
            );
        }
    }

    info!("[Day 4 - Exercise 1] Result: {}", total);
    Ok(())
}

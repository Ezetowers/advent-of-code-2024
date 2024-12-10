use log2::*;
use std::fs::File;

/// Receives a Vector with characters and validates
/// if a sequence has been found
/// Arguments
/// * my_chars: Immutable reference to the array being analyzed
/// * sequence: Immutable Sequence to be checked
/// * curr_pointer: Position in my_chars from where analysis should start
/// * offset: Position in my_chars and sequence associated with the characters already analyzed
/// Return values, in order
/// * true if sequence has been found in my_char, false if the sequence was not found
pub fn check_sequence(
    my_chars: &Vec<char>,
    sequence: &Vec<char>,
    curr_pointer: &mut usize,
    offset: &mut usize,
) -> bool {
    let mut sequence_found = false;

    if my_chars[*curr_pointer + *offset] == sequence[*offset] {
        if *offset == (sequence.len() - 1) {
            trace!("Sequence {:#?} found", sequence);
            *curr_pointer += *offset + 1;
            *offset = 0;
            sequence_found = true;
        } else {
            *offset += 1;
        }
    } else {
        // mul not found, advance pointers and start all over again
        // Only offset needs to be reset
        *curr_pointer += *offset + 1;
        *offset = 0;
    }
    sequence_found
}

pub fn setup_logger() -> log2::Handle {
    let log_level = match std::env::var("LOG_LEVEL") {
        Ok(val) => val,
        Err(_) => "info".to_string(),
    };
    log2::stdout().module(false).level(log_level).start()
}

pub fn setup_input() -> std::io::Result<File> {
    let input_path = match std::env::var("INPUT_PATH") {
        Ok(val) => val,
        Err(_) => panic!("Invalid INPUT_PATH. Check if path exists"),
    };
    File::open(&input_path)
}

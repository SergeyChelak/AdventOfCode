use std::fs::File;
use std::io::{self, Read};

use advent_of_code::file_to_string_array;

pub fn read_file_as_bytes(file_name: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_name)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn read_file_as_chars(file_name: &str) -> io::Result<Vec<char>> {
    Ok(read_file_as_bytes(file_name)?
        .iter()
        .map(|val| *val as char)
        .collect())
}

pub fn read_file_as_lines(file_name: &str) -> io::Result<Vec<String>> {
    file_to_string_array(file_name)
}

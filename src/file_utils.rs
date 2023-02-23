use std::fs::File;
use std::io::{self, Read};


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
        .collect()
    )
}

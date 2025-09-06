use serde_derive::{Deserialize, Serialize};

pub const AOC_CONFIG_FILE: &str = "aoc.toml";

#[derive(Serialize, Deserialize)]
pub struct AocConfig {
    pub puzzle: PuzzleConfig,
}

#[derive(Serialize, Deserialize)]
pub struct PuzzleConfig {
    pub year: Option<usize>,
    pub day: Option<usize>,
}

// shared
pub fn file_to_string_array<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Vec<String>> {
    let arr = std::fs::read_to_string(path)?
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    Ok(arr)
}

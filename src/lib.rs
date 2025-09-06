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

impl PuzzleConfig {
    pub fn with(year: usize, day: usize) -> PuzzleConfig {
        PuzzleConfig {
            year: Some(year),
            day: Some(day),
        }
    }
}

pub fn load_aoc_config() -> std::io::Result<AocConfig> {
    let content = std::fs::read_to_string(AOC_CONFIG_FILE)?;
    let config = toml::from_str(&content)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))?;
    Ok(config)
}

pub fn write_aoc_config(config: AocConfig) -> std::io::Result<()> {
    let content = toml::to_string(&config)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))?;
    str_to_file(AOC_CONFIG_FILE, &content)
}

// shared
pub fn file_to_string_array<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Vec<String>> {
    let arr = std::fs::read_to_string(path)?
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    Ok(arr)
}

pub fn str_to_file<P: AsRef<std::path::Path>>(path: P, output: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    std::io::Write::write_all(&mut file, output.as_bytes())?;
    std::io::Write::flush(&mut file)
}

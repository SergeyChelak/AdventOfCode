use std::path::{Path, PathBuf};

use crate::generror::GenError;

pub struct Context {
    current_dir: PathBuf,
    year: usize,
    day: Option<usize>,
}

impl Context {
    pub fn create() -> Result<Context, GenError> {
        let current_dir = std::env::current_dir().map_err(GenError::from)?;
        let mut year: Option<usize> = None;
        let mut day: Option<usize> = None;
        for arg in std::env::args() {
            let is_year = arg.starts_with("-y");
            let is_day = arg.starts_with("-d");
            if !(is_year || is_day) {
                continue;
            }
            let val = arg[2..].parse::<usize>().map_err(GenError::from)?;
            if is_year {
                year = Some(val);
                continue;
            }
            if is_day {
                day = Some(val);
                continue;
            }
        }

        let Some(year) = year else {
            return Err(GenError::new("Year '-y' parameter is missing"));
        };

        let ctxt = Context {
            current_dir,
            year,
            day,
        };

        Ok(ctxt)
    }

    pub fn year(&self) -> usize {
        self.year
    }

    pub fn is_marker_file_exists(&self) -> bool {
        self.marker_file_path().exists()
    }

    fn marker_file_path(&self) -> PathBuf {
        extend_path(&self.current_dir, "aoc.toml")
    }

    fn source_folder(&self) -> PathBuf {
        extend_path(&self.current_dir, "src")
    }

    pub fn year_folder(&self) -> PathBuf {
        extend_path(&self.source_folder(), format!("aoc{}", self.year))
    }

    pub fn year_mod_file_path(&self) -> PathBuf {
        extend_path(&self.year_folder(), "mod.rs")
    }

    pub fn main_file_path(&self) -> PathBuf {
        extend_path(&self.source_folder(), "main.rs")
    }
}

fn extend_path<T: AsRef<str>>(root: &Path, elem: T) -> PathBuf {
    let mut path = root.to_path_buf();
    path.push(elem.as_ref());
    path
}

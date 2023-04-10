use super::solution::Solution;

pub mod day_01;
pub use day_01::*;

pub fn all_days() -> std::io::Result<Vec<Box<dyn Solution>>> {
    Ok(vec![
        Box::new(AoC2016_01::new()?)
    ])
}

pub fn last_day() -> std::io::Result<Box<dyn Solution>> {
    Ok(
        Box::new(AoC2016_01::new()?)
    )
}
use super::solution::Solution;

mod day_01;
use day_01::*;

mod day_02;
use day_02::*;

pub fn all_days() -> std::io::Result<Vec<Box<dyn Solution>>> {
    Ok(vec![
        Box::new(AoC2017_01::new()?),
        Box::new(AoC2017_02::new()?),
    ])
}

pub fn last_day() -> std::io::Result<Box<dyn Solution>> {
    Ok(Box::new(AoC2017_02::new()?))
}

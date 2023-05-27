use super::solution::Solution;

mod day_01;
use day_01::*;

mod day_02;
use day_02::*;

mod day_03;
use day_03::*;

mod day_04;
use day_04::*;

mod day_05;
use day_05::*;

mod day_06;
use day_06::*;

pub fn all_days() -> std::io::Result<Vec<Box<dyn Solution>>> {
    Ok(vec![
        Box::new(AoC2017_01::new()?),
        Box::new(AoC2017_02::new()?),
        Box::new(AoC2017_03::new()?),
        Box::new(AoC2017_04::new()?),
        Box::new(AoC2017_05::new()?),
        Box::new(AoC2017_06::new()?),
    ])
}

pub fn last_day() -> std::io::Result<Box<dyn Solution>> {
    Ok(Box::new(AoC2017_06::new()?))
}

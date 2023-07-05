use super::solution::Solution;

mod day_01;
use day_01::*;

pub fn all_days() -> std::io::Result<Vec<Box<dyn Solution>>> {
    Ok(vec![
        Box::new(AoC2018_01::new()?),
        // Box::new(AoC2018_02::new()?),
        // Box::new(AoC2018_03::new()?),
        // Box::new(AoC2018_04::new()?),
        // Box::new(AoC2018_05::new()?),
        // Box::new(AoC2018_06::new()?),
        // Box::new(AoC2018_07::new()?),
        // Box::new(AoC2018_08::new()?),
        // Box::new(AoC2018_09::new()?),
        // Box::new(AoC2018_10::new()?),
        // Box::new(AoC2018_11::new()?),
        // Box::new(AoC2018_12::new()?),
        // Box::new(AoC2018_13::new()?),
        // Box::new(AoC2018_14::new()?),
        // Box::new(AoC2018_15::new()?),
        // Box::new(AoC2018_16::new()?),
        // Box::new(AoC2018_17::new()?),
        // Box::new(AoC2018_18::new()?),
        // Box::new(AoC2018_19::new()?),
        // Box::new(AoC2018_20::new()?),
        // Box::new(AoC2018_21::new()?),
        // Box::new(AoC2018_22::new()?),
        // Box::new(AoC2018_23::new()?),
        // Box::new(AoC2018_24::new()?),
        // Box::new(AoC2018_25::new()?),
    ])
}

pub fn last_day() -> std::io::Result<Box<dyn Solution>> {
    Ok(Box::new(AoC2018_01::new()?))
}
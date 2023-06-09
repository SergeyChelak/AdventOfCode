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

mod day_07;
use day_07::*;

mod day_08;
use day_08::*;

mod day_09;
use day_09::*;

mod day_10;
use day_10::*;

mod day_11;
use day_11::*;

mod day_12;
use day_12::*;

mod day_13;
use day_13::*;

pub fn all_days() -> std::io::Result<Vec<Box<dyn Solution>>> {
    Ok(vec![
        Box::new(AoC2017_01::new()?),
        Box::new(AoC2017_02::new()?),
        Box::new(AoC2017_03::new()?),
        Box::new(AoC2017_04::new()?),
        Box::new(AoC2017_05::new()?),
        Box::new(AoC2017_06::new()?),
        Box::new(AoC2017_07::new()?),
        Box::new(AoC2017_08::new()?),
        Box::new(AoC2017_09::new()?),
        Box::new(AoC2017_10::new()?),
        Box::new(AoC2017_11::new()?),
        Box::new(AoC2017_12::new()?),
        Box::new(AoC2017_13::new()?),
    ])
}

pub fn last_day() -> std::io::Result<Box<dyn Solution>> {
    Ok(Box::new(AoC2017_13::new()?))
}

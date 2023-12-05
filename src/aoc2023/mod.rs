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

// mod day_06;
// use day_06::*;

// mod day_07;
// use day_07::*;

// mod day_08;
// use day_08::*;

// mod day_09;
// use day_09::*;

// mod day_10;
// use day_10::*;

// mod day_11;
// use day_11::*;

// mod day_12;
// use day_12::*;

// mod day_13;
// use day_13::*;

// mod day_14;
// use day_14::*;

// mod day_15;
// use day_15::*;

// mod day_16;
// use day_16::*;

// mod day_17;
// use day_17::*;

// mod day_18;
// use day_18::*;

// mod day_19;
// use day_19::*;

// mod day_20;
// use day_20::*;

// mod day_21;
// use day_21::*;

// mod day_22;
// use day_22::*;

// mod day_23;
// use day_23::*;

// mod day_24;
// use day_24::*;

// mod day_25;
// use day_25::*;

pub fn all_days() -> std::io::Result<Vec<Box<dyn Solution>>> {
    Ok(vec![
        Box::new(AoC2023_01::new()?),
        Box::new(AoC2023_02::new()?),
        Box::new(AoC2023_03::new()?),
        Box::new(AoC2023_04::new()?),
        Box::new(AoC2023_05::new()?),
        // Box::new(AoC2023_06::new()?),
        // Box::new(AoC2023_07::new()?),
        // Box::new(AoC2023_08::new()?),
        // Box::new(AoC2023_09::new()?),
        // Box::new(AoC2023_10::new()?),
        // Box::new(AoC2023_11::new()?),
        // Box::new(AoC2023_12::new()?),
        // Box::new(AoC2023_13::new()?),
        // Box::new(AoC2023_14::new()?),
        // Box::new(AoC2023_15::new()?),
        // Box::new(AoC2023_16::new()?),
        // Box::new(AoC2023_17::new()?),
        // Box::new(AoC2023_18::new()?),
        // Box::new(AoC2023_19::new()?),
        // Box::new(AoC2023_20::new()?),
        // Box::new(AoC2023_21::new()?),
        // Box::new(AoC2023_22::new()?),
        // Box::new(AoC2023_23::new()?),
        // Box::new(AoC2023_24::new()?),
        // Box::new(AoC2023_25::new()?),
    ])
}

pub fn last_day() -> std::io::Result<Box<dyn Solution>> {
    Ok(Box::new(AoC2023_05::new()?))
}

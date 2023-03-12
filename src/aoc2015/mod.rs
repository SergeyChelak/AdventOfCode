use super::solution::Solution;

pub mod day_01;
pub use day_01::*;

pub mod day_02;
pub use day_02::*;

pub mod day_03;
pub use day_03::*;

pub mod day_04;
pub use day_04::*;

pub mod day_05;
pub use day_05::*;

pub mod day_06;
pub use day_06::*;

pub mod day_07;
pub use day_07::*;

pub mod day_08;
pub use day_08::*;

pub mod day_09;
pub use day_09::*;

pub mod day_10;
pub use day_10::*;

pub mod day_11;
pub use day_11::*;

pub mod day_12;
pub use day_12::*;

pub mod day_13;
pub use day_13::*;

pub mod day_14;
pub use day_14::*;

pub mod day_15;
pub use day_15::*;

pub mod day_16;
pub use day_16::*;

pub mod day_17;
pub use day_17::*;

pub mod day_18;
pub use day_18::*;

pub mod day_19;
pub use day_19::*;

pub mod day_20;
pub use day_20::*;

pub mod day_21;
pub use day_21::*;

pub fn all_days() -> std::io::Result<Vec<Box<dyn Solution>>> {
    Ok(vec![
        Box::new(AoC2015_01::new()?),
        Box::new(AoC2015_02::new()?),
        Box::new(AoC2015_03::new()?),
        Box::new(AoC2015_04::new()?),
        Box::new(AoC2015_05::new()?),
        Box::new(AoC2015_06::new()?),
        Box::new(AoC2015_07::new()?),
        Box::new(AoC2015_08::new()?),
        Box::new(AoC2015_09::new()?),
        Box::new(AoC2015_10::new()?),
        Box::new(AoC2015_11::new()?),
        Box::new(AoC2015_12::new()?),
        Box::new(AoC2015_13::new()?),
        Box::new(AoC2015_14::new()?),
        Box::new(AoC2015_15::new()?),
        Box::new(AoC2015_16::new()?),
        Box::new(AoC2015_17::new()?),
        Box::new(AoC2015_18::new()?),
        Box::new(AoC2015_19::new()?),
        Box::new(AoC2015_20::new()?),
        Box::new(AoC2015_21::new()?)
    ])
}

pub fn last_day() -> std::io::Result<Box<dyn Solution>> {
    Ok(
        Box::new(AoC2015_21::new()?)
    )
}
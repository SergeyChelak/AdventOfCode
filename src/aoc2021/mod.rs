
use super::solution::{PuzzleFactory, PuzzleFactoryMethod};

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

mod day_14;
use day_14::*;

mod day_15;
use day_15::*;

mod day_16;
use day_16::*;

mod day_17;
use day_17::*;

mod day_18;
use day_18::*;

mod day_19;
use day_19::*;

mod day_20;
use day_20::*;

// GENERATOR_MARKER: DAY_MOD_USE

pub fn puzzle_factory_2021() -> PuzzleFactory {
    let producers: Vec<&'static PuzzleFactoryMethod> = vec![
        &|| Ok(Box::new(AoC2021_01::new()?)),
        &|| Ok(Box::new(AoC2021_02::new()?)),
        &|| Ok(Box::new(AoC2021_03::new()?)),
        &|| Ok(Box::new(AoC2021_04::new()?)),
        &|| Ok(Box::new(AoC2021_05::new()?)),
        &|| Ok(Box::new(AoC2021_06::new()?)),
        &|| Ok(Box::new(AoC2021_07::new()?)),
        &|| Ok(Box::new(AoC2021_08::new()?)),
        &|| Ok(Box::new(AoC2021_09::new()?)),
        &|| Ok(Box::new(AoC2021_10::new()?)),
        &|| Ok(Box::new(AoC2021_11::new()?)),
        &|| Ok(Box::new(AoC2021_12::new()?)),
        &|| Ok(Box::new(AoC2021_13::new()?)),
        &|| Ok(Box::new(AoC2021_14::new()?)),
        &|| Ok(Box::new(AoC2021_15::new()?)),
        &|| Ok(Box::new(AoC2021_16::new()?)),
        &|| Ok(Box::new(AoC2021_17::new()?)),
        &|| Ok(Box::new(AoC2021_18::new()?)),
        &|| Ok(Box::new(AoC2021_19::new()?)),
        &|| Ok(Box::new(AoC2021_20::new()?)),
        // GENERATOR_MARKER: FACTORY_DAY
    ];
    PuzzleFactory::new(2021, producers)
}
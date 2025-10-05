use super::solution::{PuzzleFactory, PuzzleFactoryMethod};

mod day_01;
use day_01::*;

mod day_02;
use day_02::*;

mod day_03;
use day_03::*;

mod day_05;
use day_05::*;

mod day_04;
use day_04::*;

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

mod day_21;
use day_21::*;

mod day_22;
use day_22::*;

mod day_23;
use day_23::*;

// GENERATOR_MARKER: DAY_MOD_USE

pub fn puzzle_factory_2020() -> PuzzleFactory {
    let producers: Vec<&'static PuzzleFactoryMethod> = vec![
        &|| Ok(Box::new(AoC2020_01::new()?)),
        &|| Ok(Box::new(AoC2020_02::new()?)),
        &|| Ok(Box::new(AoC2020_03::new()?)),
        &|| Ok(Box::new(AoC2020_04::new()?)),
        &|| Ok(Box::new(AoC2020_05::new()?)),
        &|| Ok(Box::new(AoC2020_06::new()?)),
        &|| Ok(Box::new(AoC2020_07::new()?)),
        &|| Ok(Box::new(AoC2020_08::new()?)),
        &|| Ok(Box::new(AoC2020_09::new()?)),
        &|| Ok(Box::new(AoC2020_10::new()?)),
        &|| Ok(Box::new(AoC2020_11::new()?)),
        &|| Ok(Box::new(AoC2020_12::new()?)),
        &|| Ok(Box::new(AoC2020_13::new()?)),
        &|| Ok(Box::new(AoC2020_14::new()?)),
        &|| Ok(Box::new(AoC2020_15::new()?)),
        &|| Ok(Box::new(AoC2020_16::new()?)),
        &|| Ok(Box::new(AoC2020_17::new()?)),
        &|| Ok(Box::new(AoC2020_18::new()?)),
        &|| Ok(Box::new(AoC2020_19::new()?)),
        &|| Ok(Box::new(AoC2020_20::new()?)),
        &|| Ok(Box::new(AoC2020_21::new()?)),
        &|| Ok(Box::new(AoC2020_22::new()?)),
        &|| Ok(Box::new(AoC2020_23::new()?)),
        // GENERATOR_MARKER: FACTORY_DAY
    ];
    PuzzleFactory::new(2020, producers)
}
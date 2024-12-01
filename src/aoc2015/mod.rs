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

mod day_21;
use day_21::*;

mod day_22;
use day_22::*;

mod day_23;
use day_23::*;

mod day_24;
use day_24::*;

mod day_25;
use day_25::*;

pub fn puzzle_factory_2015() -> PuzzleFactory {
    let producers: Vec<&'static PuzzleFactoryMethod> = vec![
        &|| Ok(Box::new(AoC2015_01::new()?)),
        &|| Ok(Box::new(AoC2015_02::new()?)),
        &|| Ok(Box::new(AoC2015_03::new()?)),
        &|| Ok(Box::new(AoC2015_04::new()?)),
        &|| Ok(Box::new(AoC2015_05::new()?)),
        &|| Ok(Box::new(AoC2015_06::new()?)),
        &|| Ok(Box::new(AoC2015_07::new()?)),
        &|| Ok(Box::new(AoC2015_08::new()?)),
        &|| Ok(Box::new(AoC2015_09::new()?)),
        &|| Ok(Box::new(AoC2015_10::new()?)),
        &|| Ok(Box::new(AoC2015_11::new()?)),
        &|| Ok(Box::new(AoC2015_12::new()?)),
        &|| Ok(Box::new(AoC2015_13::new()?)),
        &|| Ok(Box::new(AoC2015_14::new()?)),
        &|| Ok(Box::new(AoC2015_15::new()?)),
        &|| Ok(Box::new(AoC2015_16::new()?)),
        &|| Ok(Box::new(AoC2015_17::new()?)),
        &|| Ok(Box::new(AoC2015_18::new()?)),
        &|| Ok(Box::new(AoC2015_19::new()?)),
        &|| Ok(Box::new(AoC2015_20::new()?)),
        &|| Ok(Box::new(AoC2015_21::new()?)),
        &|| Ok(Box::new(AoC2015_22::new()?)),
        &|| Ok(Box::new(AoC2015_23::new()?)),
        &|| Ok(Box::new(AoC2015_24::new()?)),
        &|| Ok(Box::new(AoC2015_25::new()?)),
    ];
    PuzzleFactory::new(2015, producers)
}

use super::solution::{PuzzleFactory, PuzzleFactoryMethod};

mod machine;

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

pub fn puzzle_factory_2018() -> PuzzleFactory {
    let producers: Vec<&'static PuzzleFactoryMethod> = vec![
        &|| Ok(Box::new(AoC2018_01::new()?)),
        &|| Ok(Box::new(AoC2018_02::new()?)),
        &|| Ok(Box::new(AoC2018_03::new()?)),
        &|| Ok(Box::new(AoC2018_04::new()?)),
        &|| Ok(Box::new(AoC2018_05::new()?)),
        &|| Ok(Box::new(AoC2018_06::new()?)),
        &|| Ok(Box::new(AoC2018_07::new()?)),
        &|| Ok(Box::new(AoC2018_08::new()?)),
        &|| Ok(Box::new(AoC2018_09::new()?)),
        &|| Ok(Box::new(AoC2018_10::new()?)),
        &|| Ok(Box::new(AoC2018_11::new()?)),
        &|| Ok(Box::new(AoC2018_12::new()?)),
        &|| Ok(Box::new(AoC2018_13::new()?)),
        &|| Ok(Box::new(AoC2018_14::new()?)),
        &|| Ok(Box::new(AoC2018_15::new()?)),
        &|| Ok(Box::new(AoC2018_16::new()?)),
        &|| Ok(Box::new(AoC2018_17::new()?)),
        &|| Ok(Box::new(AoC2018_18::new()?)),
        &|| Ok(Box::new(AoC2018_19::new()?)),
        &|| Ok(Box::new(AoC2018_20::new()?)),
        &|| Ok(Box::new(AoC2018_21::new()?)),
        &|| Ok(Box::new(AoC2018_22::new()?)),
        &|| Ok(Box::new(AoC2018_23::new()?)),
        &|| Ok(Box::new(AoC2018_24::new()?)),
        &|| Ok(Box::new(AoC2018_25::new()?)),
    ];
    PuzzleFactory::new(2018, producers)
}

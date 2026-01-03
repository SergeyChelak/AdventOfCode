
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
        // GENERATOR_MARKER: FACTORY_DAY
    ];
    PuzzleFactory::new(2021, producers)
}
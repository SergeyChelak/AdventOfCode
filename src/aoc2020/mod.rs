
use super::solution::{PuzzleFactory, PuzzleFactoryMethod};

mod day_01;
use day_01::*;

mod day_02;
use day_02::*;

mod day_03;
use day_03::*;

// GENERATOR_MARKER: DAY_MOD_USE

pub fn puzzle_factory_2020() -> PuzzleFactory {
    let producers: Vec<&'static PuzzleFactoryMethod> = vec![
        &|| Ok(Box::new(AoC2020_01::new()?)),
        &|| Ok(Box::new(AoC2020_02::new()?)),
        &|| Ok(Box::new(AoC2020_03::new()?)),
        // GENERATOR_MARKER: FACTORY_DAY
    ];
    PuzzleFactory::new(2020, producers)
}

use super::solution::{PuzzleFactory, PuzzleFactoryMethod};

mod day_01;
use day_01::*;

mod day_02;
use day_02::*;

// GENERATOR_MARKER: DAY_MOD_USE

pub fn puzzle_factory_2021() -> PuzzleFactory {
    let producers: Vec<&'static PuzzleFactoryMethod> = vec![
        &|| Ok(Box::new(AoC2021_01::new()?)),
        &|| Ok(Box::new(AoC2021_02::new()?)),
        // GENERATOR_MARKER: FACTORY_DAY
    ];
    PuzzleFactory::new(2021, producers)
}

use super::solution::{PuzzleFactory, PuzzleFactoryMethod};

mod day_01;
use day_01::*;

// GENERATOR_MARKER: DAY_MOD_USE

pub fn puzzle_factory_2025() -> PuzzleFactory {
    let producers: Vec<&'static PuzzleFactoryMethod> = vec![
        &|| Ok(Box::new(AoC2025_01::new()?)),
        // GENERATOR_MARKER: FACTORY_DAY
    ];
    PuzzleFactory::new(2025, producers)
}
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

pub fn puzzle_factory_2024() -> PuzzleFactory {
    let producers: Vec<&'static PuzzleFactoryMethod> = vec![
        &|| Ok(Box::new(AoC2024_01::new()?)),
        &|| Ok(Box::new(AoC2024_02::new()?)),
        &|| Ok(Box::new(AoC2024_03::new()?)),
        &|| Ok(Box::new(AoC2024_04::new()?)),
        &|| Ok(Box::new(AoC2024_05::new()?)),
        &|| Ok(Box::new(AoC2024_06::new()?)),
        &|| Ok(Box::new(AoC2024_07::new()?)),
        &|| Ok(Box::new(AoC2024_08::new()?)),
        &|| Ok(Box::new(AoC2024_09::new()?)),
        &|| Ok(Box::new(AoC2024_10::new()?)),
        &|| Ok(Box::new(AoC2024_11::new()?)),
        // &|| { Ok(Box::new(AoC2024_12::new()?)) },
        // &|| { Ok(Box::new(AoC2024_13::new()?)) },
        // &|| { Ok(Box::new(AoC2024_14::new()?)) },
        // &|| { Ok(Box::new(AoC2024_15::new()?)) },
        // &|| { Ok(Box::new(AoC2024_16::new()?)) },
        // &|| { Ok(Box::new(AoC2024_17::new()?)) },
        // &|| { Ok(Box::new(AoC2024_18::new()?)) },
        // &|| { Ok(Box::new(AoC2024_19::new()?)) },
        // &|| { Ok(Box::new(AoC2024_20::new()?)) },
        // &|| { Ok(Box::new(AoC2024_21::new()?)) },
        // &|| { Ok(Box::new(AoC2024_22::new()?)) },
        // &|| { Ok(Box::new(AoC2024_23::new()?)) },
        // &|| { Ok(Box::new(AoC2024_24::new()?)) },
        // &|| { Ok(Box::new(AoC2024_25::new()?)) },
    ];
    PuzzleFactory::new(2024, producers)
}

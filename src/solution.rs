use std::io;

pub trait Solution {
    fn part_one(&self) -> String {
        "Part #1 isn't implemented yet".to_string()
    }

    fn part_two(&self) -> String {
        "Part #2 isn't implemented yet".to_string()
    }

    fn description(&self) -> String {
        "* Unnamed solution *".to_string()
    }
}

pub enum PuzzleFactoryError {
    NotFound,
    InitializationFailed,
}

pub type PuzzleFactoryResult<T> = Result<T, PuzzleFactoryError>;
pub type PuzzleFactoryMethod = dyn Fn() -> io::Result<Box<dyn Solution>>;
pub struct PuzzleFactory {
    year: usize,
    producers: Vec<&'static PuzzleFactoryMethod>,
}

impl PuzzleFactory {
    pub fn new(year: usize, producers: Vec<&'static PuzzleFactoryMethod>) -> Self {
        Self { year, producers }
    }

    pub fn puzzle(&self, year: usize, day: usize) -> PuzzleFactoryResult<Box<dyn Solution>> {
        if year != self.year || day == 0 || day > 25 {
            return Err(PuzzleFactoryError::NotFound);
        }
        let Some(puzzle) = self.producers.get(day - 1) else {
            return Err(PuzzleFactoryError::NotFound);
        };
        puzzle().map_err(|_| PuzzleFactoryError::InitializationFailed)
    }
}

pub struct AggregatedFactory {
    factories: Vec<PuzzleFactory>,
}

impl AggregatedFactory {
    pub fn new() -> Self {
        Self {
            factories: Vec::new(),
        }
    }

    pub fn add_factory(&mut self, factory: PuzzleFactory) {
        self.factories.push(factory);
    }

    pub fn puzzle(
        &self,
        year: usize,
        day: usize,
    ) -> Option<PuzzleFactoryResult<Box<dyn Solution>>> {
        for factory in &self.factories {
            let result = factory.puzzle(year, day);
            if let Err(err) = &result {
                if matches!(err, PuzzleFactoryError::NotFound) {
                    continue;
                }
            }
            return Some(result);
        }
        None
    }
}

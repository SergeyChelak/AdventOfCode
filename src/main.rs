use std::io;
use std::ops::Deref;
use std::time::Instant;
mod solution;
mod utils;

mod aoc2015;
mod aoc2016;
mod aoc2017;
mod aoc2018;
mod aoc2023;
use solution::Solution;

fn main() -> io::Result<()> {
    println!("Advent of Code");
    let args: Vec<String> = std::env::args().collect();

    let values = (args.get(1), args.get(2));
    match values {
        (Some(year), None) => {
            let solutions = collection(year)?;
            run_collection(solutions);
        }
        (Some(year), Some(day)) => {
            let solutions = collection(year)?;
            let day = day
                .parse::<usize>()
                .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
            execute(
                solutions
                    .get(day - 1)
                    .expect("Day number should be between 1 and 25")
                    .deref(),
            );
        }
        _ => {
            if let Ok(day) = &aoc2023::last_day() {
                execute(day.deref());
            }
        }
    }
    Ok(())
}

fn collection(year: &str) -> io::Result<Vec<Box<dyn Solution>>> {
    match year {
        "2015" => aoc2015::all_days(),
        "2016" => aoc2016::all_days(),
        "2017" => aoc2017::all_days(),
        "2018" => aoc2018::all_days(),
        "2023" => aoc2023::all_days(),
        _ => Ok(vec![]),
    }
}

fn run_collection(days: Vec<Box<dyn Solution>>) {
    days.iter().for_each(|x| execute(x.deref()));
}

fn execute(solution: &dyn Solution) {
    println!();
    println!("{}", solution.description());
    let measure = |part: u8, proc: &dyn Fn() -> String| {
        let now = Instant::now();
        let result = proc();
        let duration = now.elapsed().as_millis();
        let title = format!("{} ms for part {}", duration, part);
        println!("{:>30}: {}", title, result);
    };
    measure(1, &|| solution.part_one());
    measure(2, &|| solution.part_two());
}

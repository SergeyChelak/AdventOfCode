use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

type Int = isize;
type Point = Point2d<Int>;

const COST_A: Int = 3;
const COST_B: Int = 1;

struct Machine {
    step_a: Point,
    step_b: Point,
    target: Point,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let parse = |s: &str| -> Point {
            let (_, s) = s.split_once(": ").expect("Invalid format {s}");
            let coords = s
                .split(", ")
                .map(|x| &x[2..])
                .map(|x| {
                    x.parse::<Int>()
                        .unwrap_or_else(|_| panic!("Non numeric value {x}"))
                })
                .collect::<Vec<_>>();
            assert_eq!(2, coords.len());
            Point::new(coords[0], coords[1])
        };

        let lines = value
            .split('\n')
            .map(|x| x.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        assert_eq!(3, lines.len());

        Self {
            step_a: parse(lines[0]),
            step_b: parse(lines[1]),
            target: parse(lines[2]),
        }
    }
}

pub struct AoC2024_13 {
    input: Vec<Machine>,
}

impl AoC2024_13 {
    pub fn new() -> io::Result<Self> {
        let input = read_to_string("input/aoc2024_13")?;
        Ok(Self::with_data(&input))
    }

    fn with_data(input: &str) -> Self {
        let input = input.split("\n\n").map(Machine::from).collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2024_13 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .filter_map(get_cost)
            .sum::<Int>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let val = 10000000000000;
        self.input
            .iter()
            .map(|machine| {
                let Point { x, y } = machine.target;
                Machine {
                    target: Point::new(x + val, y + val),
                    ..*machine
                }
            })
            .filter_map(|m| get_cost(&m))
            .sum::<Int>()
            .to_string()
    }

    fn description(&self) -> String {
        "2024/Day 13: Claw Contraption".to_string()
    }
}

#[derive(Clone, Copy)]
struct Route {
    a_taps: Int,
    b_taps: Int,
}

impl Route {
    fn cost(&self) -> Int {
        self.a_taps * COST_A + self.b_taps * COST_B
    }

    fn position(&self, machine: &Machine) -> Point {
        let x = self.a_taps * machine.step_a.x + self.b_taps * machine.step_b.x;
        let y = self.a_taps * machine.step_a.y + self.b_taps * machine.step_b.y;
        Point::new(x, y)
    }

    fn zero() -> Self {
        Self {
            a_taps: 0,
            b_taps: 0,
        }
    }

    fn by_a_tap(&self) -> Self {
        Self {
            a_taps: self.a_taps + 1,
            b_taps: self.b_taps,
        }
    }

    fn by_b_tap(&self) -> Self {
        Self {
            a_taps: self.a_taps,
            b_taps: self.b_taps + 1,
        }
    }
}

fn _get_cost(machine: &Machine) -> Option<Int> {
    let mut price_map = HashMap::new();
    price_map.insert(Point::new(0, 0), 0);
    let target = machine.target;
    let mut queue = Vec::new();
    queue.push(Route::zero());
    while let Some(route) = queue.pop() {
        let routes = [route.by_a_tap(), route.by_b_tap()];
        for next in &routes {
            let pos = next.position(machine);
            if pos.x > target.x || pos.y > target.y {
                continue;
            }
            let cost = next.cost();
            let mut is_better = true;
            if let Some(old_cost) = price_map.get(&pos) {
                is_better = *old_cost > cost;
            }
            if is_better {
                price_map.insert(pos, cost);
                queue.push(*next);
            }

            if pos == machine.target {
                return Some(cost);
            }
        }
    }
    price_map.get(&machine.target).copied()
}

fn get_cost(machine: &Machine) -> Option<Int> {
    let Point { x: tx, y: ty } = machine.target;
    let Point { x: ax, y: ay } = machine.step_a;
    let Point { x: bx, y: by } = machine.step_b;
    let Some(beta) = ({
        let nom = ty * ax - ay * tx;
        let denom = ax * by - ay * bx;
        if nom % denom != 0 {
            None
        } else {
            let val = nom / denom;
            if val < 0 {
                return None;
            }
            Some(val)
        }
    }) else {
        return None;
    };

    let Some(alpha) = ({
        let nom = tx - bx * beta;
        if nom < 0 {
            None
        } else if nom % ax != 0 {
            None
        } else {
            Some(nom / ax)
        }
    }) else {
        return None;
    };

    let route = Route {
        a_taps: alpha,
        b_taps: beta,
    };

    Some(route.cost())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2024_13_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2024_13_parse_machine() {
        let input = "Button A: X+11, Y+67\nButton B: X+17, Y+16\nPrize: X=2413, Y=6293";
        let machine = Machine::from(input);
        assert_eq!(machine.step_a.x, 11);
        assert_eq!(machine.step_a.y, 67);

        assert_eq!(machine.step_b.x, 17);
        assert_eq!(machine.step_b.y, 16);

        assert_eq!(machine.target.x, 2413);
        assert_eq!(machine.target.y, 6293);
    }

    #[test]
    fn aoc2024_13_case_1() {
        let puzzle = make_test_solution();
        assert_eq!(puzzle.part_one(), "480")
    }

    #[test]
    fn aoc2024_13_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "37901");
        Ok(())
    }

    #[test]
    fn aoc2024_13_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "77407675412647");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2024_13> {
        AoC2024_13::new()
    }

    fn make_test_solution() -> AoC2024_13 {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        AoC2024_13::with_data(input)
    }
}

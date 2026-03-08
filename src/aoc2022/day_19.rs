use crate::solution::Solution;
use std::collections::HashMap;

use std::io;

type Int = i32;

struct RobotCost {
    ore: Int,
    clay: Int,
    obsidian: Int,
}

struct Blueprint {
    id: Int,
    ore_robot_cost: RobotCost,
    clay_robot_cost: RobotCost,
    obsidian_robot_cost: RobotCost,
    geode_robot_cost: RobotCost,
}

pub struct AoC2022_19 {
    input: Vec<Blueprint>,
}

impl AoC2022_19 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2022_19")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        Self::parse_lines(&data.lines().collect::<Vec<_>>())
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines
            .iter()
            .map(|s| s.as_ref())
            .map(Blueprint::from)
            .collect::<Vec<_>>();
        Self { input }
    }
}

impl Solution for AoC2022_19 {
    fn part_one(&self) -> String {
        self.input
            .iter()
            .map(|b| b.quality_level(24))
            .sum::<Int>()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.input
            .iter()
            .take(3)
            .map(|b| b.simulate_geode_open(32))
            .product::<Int>()
            .to_string()
    }

    fn description(&self) -> String {
        "Day 19: Not Enough Minerals".to_string()
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    ore: Int,
    clay: Int,
    obsidian: Int,
    geodes: Int,
    ore_robots: Int,
    clay_robots: Int,
    obsidian_robots: Int,
    geode_robots: Int,
    time_left: Int,
}

impl Blueprint {
    fn simulate_geode_open(&self, time: usize) -> Int {
        let initial_state = State {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            time_left: time as Int,
        };

        let mut memo = HashMap::new();
        let mut max_geodes = 0;

        // The maximum amount of any resource we could possibly spend in one minute
        let max_ore_cost = [
            self.ore_robot_cost.ore,
            self.clay_robot_cost.ore,
            self.obsidian_robot_cost.ore,
            self.geode_robot_cost.ore,
        ]
        .into_iter()
        .max()
        .unwrap();

        self.dfs(initial_state, max_ore_cost, &mut max_geodes, &mut memo);
        max_geodes
    }

    fn dfs(
        &self,
        mut state: State,
        max_ore: Int,
        max_geodes: &mut Int,
        memo: &mut HashMap<State, Int>,
    ) {
        // 1. Resource Capping (Normalization)
        // If we have more robots than the max cost of a resource, we don't need more.
        // If we have more resource than we could possibly spend in remaining time, cap it.
        state.ore = state
            .ore
            .min(state.time_left * max_ore - state.ore_robots * (state.time_left - 1));
        state.clay = state.clay.min(
            state.time_left * self.obsidian_robot_cost.clay
                - state.clay_robots * (state.time_left - 1),
        );
        state.obsidian = state.obsidian.min(
            state.time_left * self.geode_robot_cost.obsidian
                - state.obsidian_robots * (state.time_left - 1),
        );

        // 2. Check Memo
        if let Some(&best) = memo.get(&state) {
            if best >= state.geodes {
                return;
            }
        }
        memo.insert(state, state.geodes);

        // 3. Update Global Best
        *max_geodes = (*max_geodes).max(state.geodes);

        if state.time_left == 0 {
            return;
        }

        // 4. Theoretical Max Pruning (Greedy Upper Bound)
        let t = state.time_left;
        let potential = state.geodes + (state.geode_robots * t) + (t * (t - 1) / 2);
        if potential <= *max_geodes {
            return;
        }

        // 5. Branching Logic (Prioritize higher-tier robots)

        // BUILD GEODE ROBOT
        if state.ore >= self.geode_robot_cost.ore
            && state.obsidian >= self.geode_robot_cost.obsidian
        {
            let mut next = state;
            next.time_left -= 1;
            next.ore = state.ore + state.ore_robots - self.geode_robot_cost.ore;
            next.obsidian = state.obsidian + state.obsidian_robots - self.geode_robot_cost.obsidian;
            next.clay += state.clay_robots;
            next.geodes += state.geode_robots;
            next.geode_robots += 1;
            self.dfs(next, max_ore, max_geodes, memo);
            return; // Greedy Optimization: If you can build a geode robot, don't consider other paths
        }

        // BUILD OBSIDIAN ROBOT
        if state.obsidian_robots < self.geode_robot_cost.obsidian
            && state.ore >= self.obsidian_robot_cost.ore
            && state.clay >= self.obsidian_robot_cost.clay
        {
            let mut next = state;
            next.time_left -= 1;
            next.ore = state.ore + state.ore_robots - self.obsidian_robot_cost.ore;
            next.clay = state.clay + state.clay_robots - self.obsidian_robot_cost.clay;
            next.obsidian += state.obsidian_robots;
            next.geodes += state.geode_robots;
            next.obsidian_robots += 1;
            self.dfs(next, max_ore, max_geodes, memo);
        }

        // BUILD CLAY ROBOT
        if state.clay_robots < self.obsidian_robot_cost.clay
            && state.ore >= self.clay_robot_cost.ore
        {
            let mut next = state;
            next.time_left -= 1;
            next.ore = state.ore + state.ore_robots - self.clay_robot_cost.ore;
            next.clay += state.clay_robots;
            next.obsidian += state.obsidian_robots;
            next.geodes += state.geode_robots;
            next.clay_robots += 1;
            self.dfs(next, max_ore, max_geodes, memo);
        }

        // BUILD ORE ROBOT
        if state.ore_robots < max_ore && state.ore >= self.ore_robot_cost.ore {
            let mut next = state;
            next.time_left -= 1;
            next.ore = state.ore + state.ore_robots - self.ore_robot_cost.ore;
            next.clay += state.clay_robots;
            next.obsidian += state.obsidian_robots;
            next.geodes += state.geode_robots;
            next.ore_robots += 1;
            self.dfs(next, max_ore, max_geodes, memo);
        }

        // DO NOTHING (Wait)
        // Only sensible if we didn't build a Geode robot
        let mut next = state;
        next.time_left -= 1;
        next.ore += state.ore_robots;
        next.clay += state.clay_robots;
        next.obsidian += state.obsidian_robots;
        next.geodes += state.geode_robots;
        self.dfs(next, max_ore, max_geodes, memo);
    }

    fn quality_level(&self, time: usize) -> Int {
        self.id * self.simulate_geode_open(time)
    }
}

impl From<&str> for Blueprint {
    fn from(value: &str) -> Self {
        let (id_info, data) = value.split_once(": ").expect("Invalid blueprint format");

        let id = id_info
            .strip_prefix("Blueprint ")
            .expect("Invalid blueprint format")
            .parse::<Int>()
            .expect("Invalid blueprint id format");

        let robot_costs = data.split(". ").collect::<Vec<_>>();
        assert_eq!(robot_costs.len(), 4);

        let ore_robot_cost = RobotCost::from(robot_costs[0]);
        let clay_robot_cost = RobotCost::from(robot_costs[1]);
        let obsidian_robot_cost = RobotCost::from(robot_costs[2]);
        let geode_robot_cost = RobotCost::from(robot_costs[3]);

        Self {
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        }
    }
}

impl From<&str> for RobotCost {
    fn from(value: &str) -> Self {
        let mut ore = 0;
        let mut clay = 0;
        let mut obsidian = 0;

        let mut prev_token = "";

        let mut skip = true;
        for token in value.split_ascii_whitespace() {
            if token == "costs" {
                skip = false;
            }
            if skip {
                continue;
            }
            if token.starts_with("ore") {
                ore = prev_token.parse().expect("Invalid ore amount");
            }
            if token.starts_with("clay") {
                clay = prev_token.parse().expect("Invalid clay amount");
            }
            if token.starts_with("obsidian") {
                obsidian = prev_token.parse().expect("Invalid obsidian amount")
            }
            prev_token = token;
        }

        Self {
            ore,
            clay,
            obsidian,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_19_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        assert_eq!(sol.input.len(), 30);
        Ok(())
    }

    #[test]
    fn aoc2022_19_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "2301");
        Ok(())
    }

    #[test]
    fn aoc2022_19_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "10336");
        Ok(())
    }

    #[test]
    fn aoc2022_19_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "33");
    }

    fn make_solution() -> io::Result<AoC2022_19> {
        AoC2022_19::new()
    }

    fn make_test_solution() -> AoC2022_19 {
        AoC2022_19::parse_lines(&[
            "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.",
            "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.",
        ])
    }
}

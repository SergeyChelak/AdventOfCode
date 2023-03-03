use crate::solution::Solution;
use crate::utils::*;

use std::collections::HashMap;
use std::io;

struct Distance(String, String, usize);
type Graph = HashMap<(usize, usize), usize>;
type Criteria = dyn Fn(&Option<usize>, &Option<usize>) -> Option<usize>;

pub struct AoC2015_09 {
    cities_count: usize,
    graph: Graph,
}

impl AoC2015_09 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2015_09")?;
        let input = Self::parse_input(&lines);
        Ok(Self {
            cities_count: input.0,
            graph: input.1,
        })
    }

    fn parse_input(lines: &Vec<String>) -> (usize, Graph) {
        let distances = lines.iter()
            .map(|s| Self::parse_line(&s))
            .collect::<Vec<Distance>>();
        let mut cities = String2IdMapper::new();
        let mut graph = Graph::new();
        for dist in distances {
            let from = cities.get_id(&dist.0);
            let to = cities.get_id(&dist.1);
            graph.insert((from, to), dist.2);
            graph.insert((to, from), dist.2);
        }
        (cities.len(), graph)
    }

    fn parse_line(s: &str) -> Distance {
        let equation = s.split(" = ").collect::<Vec<&str>>();
        assert_eq!(equation.len(), 2, "Incorrect input string");
        let weight = equation.last()
            .unwrap()
            .parse::<usize>()
            .expect("Integer value expected after =");
        let cities = equation[0].split(" to ").collect::<Vec<&str>>();
        assert_eq!(cities.len(), 2, "Incorrect input string");
        Distance(cities[0].to_string(), cities[1].to_string(), weight)
    }

    fn permute(&self, nums: &mut Vec<usize>, pos: usize, dist: &mut Option<usize>, criteria: &Criteria) {
        fn restore(nums: &mut Vec<usize>, pos: usize) {
            for i in pos..nums.len() - 1 {
                let v = nums[i];
                nums[i] = nums[i + 1];
                nums[i + 1] = v;
            }
        }

        if pos == nums.len() - 1 {
            *dist = criteria(&self.calc_distance(nums), dist);
        }
        for i in pos..nums.len() {
            let v = nums[pos];
            nums[pos] = nums[i];
            nums[i] = v;
            self.permute(nums, pos + 1, dist, criteria);
            restore(nums, pos + 1);
        }
    }

    fn calc_distance(&self, nums: &Vec<usize>) -> Option<usize> {
        let mut sum = 0usize;
        for i in 0..nums.len() - 1 {
            if let Some(dist) = self.graph.get(&(nums[i], nums[i + 1])) {
                sum += dist;
            } else {
                return None;
            }
        }
        Some(sum)
    }

    fn find_path(&self, criteria: &Criteria) -> String {
        let mut order = vec![0usize; self.cities_count];
        for i in 0..self.cities_count {
            order[i] = i;
        }
        let mut distance = None;
        self.permute(&mut order, 0, &mut distance, criteria);
        if let Some(v) = distance {
            v.to_string()
        } else {
            "Not found".to_string()
        }
    }
}

impl Solution for AoC2015_09 {
    fn part_one(&self) -> String {
        self.find_path(&less_option)
    }

    fn part_two(&self) -> String {
        self.find_path(&bigger_option)
    }

    fn description(&self) -> String {
    	"AoC 2015/Day 9: All in a Single Night".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2015_09_input_load_test() -> io::Result<()> {
        let sol = AoC2015_09::new()?;
        assert!(sol.cities_count > 0, "Cities mapping not loaded");
        assert!(sol.graph.len() > 0, "Graph is empty");
        Ok(())
    }

    #[test]
    fn aoc2015_09_correctness() -> io::Result<()> {
        let sol = AoC2015_09::new()?;
        assert_eq!(sol.part_one(), "141");
        assert_eq!(sol.part_two(), "736");
        Ok(())
    }
}
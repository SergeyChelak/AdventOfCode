use crate::solution::Solution;
use crate::utils::*;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io;

type UInt = usize;
type Coordinate = Point2d<UInt>;
type GeologicMap = Vec<Vec<UInt>>;
type ErosionMap = Vec<Vec<RegionErosion>>;

#[derive(Copy, Clone, Debug)]
enum RegionErosion {
    Rocky,
    Wet,
    Narrow,
}

pub struct AoC2018_22 {
    depth: usize,
    target: Coordinate,
}

impl AoC2018_22 {
    pub fn new() -> io::Result<Self> {
        Ok(Self::with_parameters(9171, 7, 721))
    }

    fn with_parameters(depth: usize, x: UInt, y: UInt) -> Self {
        Self {
            depth,
            target: Coordinate::new(x, y),
        }
    }
}

impl Solution for AoC2018_22 {
    fn part_one(&self) -> String {
        let map = geologic_map(
            self.target.y + 1,
            self.target.x + 1,
            self.depth,
            self.target,
        );
        risk(&map, self.depth).to_string()
    }

    fn part_two(&self) -> String {
        let rate = 20;
        let geologic_map = geologic_map(
            1 + rate * self.target.y,
            1 + rate * self.target.x,
            self.depth,
            self.target,
        );
        let erosion_map = erosion_map(&geologic_map, self.depth);
        // dump(&erosion_map);
        find(&erosion_map, self.target)
            .map(|x| x.to_string())
            .unwrap_or("Not found".to_string())
    }

    fn description(&self) -> String {
        "AoC 2018/Day 22: Mode Maze".to_string()
    }
}

fn geologic_map(rows: usize, cols: usize, depth: UInt, target: Coordinate) -> GeologicMap {
    let (x, y) = (target.x, target.y);
    let mut regions = GeologicMap::with_capacity(rows);
    for row in 0..rows {
        let mut arr = Vec::with_capacity(cols);
        for col in 0..cols {
            if (col, row) == (0, 0) || (col, row) == (x, y) {
                arr.push(0);
                continue;
            }
            // If the region's Y coordinate is 0, the geologic index is its X coordinate times 16807.
            if row == 0 {
                arr.push(col * 16807);
                continue;
            }
            // If the region's X coordinate is 0, the geologic index is its Y coordinate times 48271.
            if col == 0 {
                arr.push(row * 48271);
                continue;
            }
            let gi_1 = erosion_level(arr[col - 1], depth);
            let gi_2 = erosion_level(regions.last().unwrap()[col], depth);
            arr.push(gi_1 * gi_2);
        }
        regions.push(arr);
    }
    regions
}

fn risk(regions: &GeologicMap, depth: UInt) -> UInt {
    regions
        .iter()
        .flatten()
        .map(|val| erosion_level(*val, depth) % 3)
        .sum()
}

fn erosion_map(geologic_map: &GeologicMap, depth: UInt) -> ErosionMap {
    geologic_map
        .iter()
        .map(|row| {
            row.iter()
                .map(|x| erosion_level(*x, depth) % 3)
                .map(|val| match val {
                    0 => RegionErosion::Rocky,
                    1 => RegionErosion::Wet,
                    2 => RegionErosion::Narrow,
                    _ => panic!("Unexpected value {val}"),
                })
                .collect::<Vec<RegionErosion>>()
        })
        .collect()
}

fn erosion_level(val: UInt, depth: UInt) -> UInt {
    (val + depth) % 20183
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Equipment {
    Neither,
    Torch,
    Gear,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    coordinate: Coordinate,
    equipment: Equipment,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct QueueItem {
    weight: UInt,
    state: State,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .weight
            .cmp(&self.weight)
            .then(self.state.cmp(&other.state))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find(erosion_map: &ErosionMap, target: Coordinate) -> Option<usize> {
    let mut weights = HashMap::<State, usize>::new();
    let start = State {
        coordinate: Coordinate::new(0, 0),
        equipment: Equipment::Torch,
    };
    weights.insert(start, 0);

    // ---- here should be functions but I'm too lazy to forward context
    let rows = erosion_map.len();
    let cols = erosion_map[rows - 1].len();
    let get_adjacent = |coord: Coordinate| {
        let mut arr = Vec::new();
        if coord.x > 0 {
            arr.push(modified_coordinate(coord, Some(Modification::Dec), None));
        }
        if coord.x < cols - 1 {
            arr.push(modified_coordinate(coord, Some(Modification::Inc), None));
        }
        if coord.y > 0 {
            arr.push(modified_coordinate(coord, None, Some(Modification::Dec)));
        }
        if coord.y < rows - 1 {
            arr.push(modified_coordinate(coord, None, Some(Modification::Inc)));
        }
        arr
    };

    let toggle_equipment = |cur: Equipment, coordinate: Coordinate| -> Equipment {
        let region = erosion_map[coordinate.y][coordinate.x];
        use Equipment::*;
        use RegionErosion::*;
        match (region, cur) {
            (Rocky, Torch) => Gear,
            (Rocky, Gear) => Torch,
            (Narrow, Torch) => Neither,
            (Narrow, Neither) => Torch,
            (Wet, Gear) => Neither,
            (Wet, Neither) => Gear,
            _ => panic!("invalid state"),
        }
    };

    let is_applicable = |equipment: Equipment, coordinate: Coordinate| -> bool {
        if coordinate == target {
            return equipment == Equipment::Torch;
        }
        let region = erosion_map[coordinate.y][coordinate.x];
        use Equipment::*;
        use RegionErosion::*;
        match region {
            Rocky => matches!(equipment, Gear | Torch),
            Narrow => matches!(equipment, Torch | Neither),
            Wet => matches!(equipment, Gear | Neither),
        }
    };
    // ----
    let mut queue: BinaryHeap<QueueItem> = BinaryHeap::new();
    queue.push(QueueItem {
        weight: 0,
        state: start,
    });
    while let Some(item) = queue.pop() {
        let state = item.state;
        if state.coordinate == target {
            return Some(item.weight);
        }
        let state_time = *weights
            .get(&state)
            .expect("that should be unreachable of something wrong with implementation");
        let adjacent = get_adjacent(state.coordinate);
        let mut equip = state.equipment;
        for region in adjacent {
            for step_time in [1, 8] {
                if is_applicable(equip, region) {
                    let adj_state = State {
                        coordinate: region,
                        equipment: equip,
                    };
                    let adj_old_time = weights.get(&adj_state).copied().unwrap_or(usize::MAX);
                    let adj_new_time = state_time + step_time;
                    if adj_new_time < adj_old_time {
                        weights.insert(adj_state, adj_new_time);
                        queue.push(QueueItem {
                            weight: adj_new_time,
                            state: adj_state,
                        });
                    }
                }
                equip = toggle_equipment(equip, state.coordinate);
            }
        }
    }
    None
}

enum Modification {
    Inc,
    Dec,
}

fn modified_coordinate(
    coordinate: Coordinate,
    x_mod: Option<Modification>,
    y_mod: Option<Modification>,
) -> Coordinate {
    let mut new = coordinate;
    let calc = |val: UInt, modification: Modification| match modification {
        Modification::Dec => val - 1,
        Modification::Inc => val + 1,
    };
    if let Some(m) = x_mod {
        new.x = calc(new.x, m);
    }
    if let Some(m) = y_mod {
        new.y = calc(new.y, m);
    }
    new
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2018_22_example1() {
        let sol = AoC2018_22::with_parameters(510, 10, 10);
        assert_eq!(sol.part_one(), "114")
    }

    #[test]
    fn aoc2018_22_example2() {
        let sol = AoC2018_22::with_parameters(510, 10, 10);
        assert_eq!(sol.part_two(), "45")
    }

    #[test]
    fn aoc2018_22_correctness() -> io::Result<()> {
        let sol = AoC2018_22::new()?;
        assert_eq!(sol.part_one(), "5786");
        assert_eq!(sol.part_two(), "986");
        Ok(())
    }
}

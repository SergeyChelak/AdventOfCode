use crate::{solution::Solution, utils::hyper_point::HyperPoint};

use std::{
    collections::{HashSet, VecDeque},
    io,
};

type Int = isize;
type Point = HyperPoint<Int>;
type PointSet = HashSet<Point>;

#[derive(Clone)]
struct Scanner {
    pole: Point,
    beacons: PointSet,
}

impl Scanner {
    fn absolute_beacons(&self) -> PointSet {
        self.beacons.iter().map(|p| p.add(&self.pole)).collect()
    }

    fn update_pole(&mut self, relative: &Self) -> bool {
        let mut offset: Option<Point> = None;
        'outer: for point in self.beacons.iter() {
            for other in relative.beacons.iter() {
                let delta = other.sub(point);
                let count = self
                    .beacons
                    .iter()
                    .map(|p| p.add(&delta))
                    .filter(|p| relative.beacons.contains(p))
                    .count();
                if count == 12 {
                    offset = Some(delta);
                    break 'outer;
                }
            }
        }
        let Some(offset) = offset else {
            return false;
        };
        self.pole = relative.pole.add(&offset);
        true
    }

    fn update_beacons(&mut self, transform: impl Fn(&Point) -> Point) {
        self.beacons = self.beacons.iter().map(transform).collect::<PointSet>();
    }

    fn rotate(&mut self, axis: Axis) {
        match axis {
            Axis::X => self.update_beacons(|p| HyperPoint(vec![p.x(), -p.z(), p.y()])),
            Axis::Y => self.update_beacons(|p| HyperPoint(vec![p.z(), p.y(), -p.x()])),
            Axis::Z => self.update_beacons(|p| HyperPoint(vec![p.y(), -p.x(), p.z()])),
        }
    }
}

impl Point {
    fn x(&self) -> Int {
        (self.0)[0]
    }

    fn y(&self) -> Int {
        (self.0)[1]
    }

    fn z(&self) -> Int {
        (self.0)[2]
    }

    fn manhattan(&self, other: &Self) -> usize {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .sum()
    }
}

#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

fn resolve_scanner_poles(input: &[PointSet]) -> Vec<Scanner> {
    let mut scanners = input
        .iter()
        .cloned()
        .map(|beacons| Scanner {
            beacons,
            pole: Point::from(vec![0; 3]),
        })
        .collect::<VecDeque<_>>();

    let mut aligned = vec![scanners.pop_back().unwrap()];

    'outer: while let Some(mut candidate) = scanners.pop_back() {
        for scanner in aligned.iter() {
            for x in 0..2 {
                for _ in 0..4 {
                    for _ in 0..4 {
                        if candidate.update_pole(scanner) {
                            aligned.push(candidate);
                            continue 'outer;
                        }
                        candidate.rotate(Axis::Y);
                    }
                    candidate.rotate(Axis::Z);
                }
                if x == 0 {
                    candidate.rotate(Axis::X);
                } else {
                    candidate.update_beacons(|p| HyperPoint(vec![p.x(), -p.y(), -p.z()]));
                }
            }
        }
        scanners.push_front(candidate);
    }
    aligned
}

pub struct AoC2021_19 {
    input: Vec<PointSet>,
}

impl AoC2021_19 {
    pub fn new() -> io::Result<Self> {
        let data = std::fs::read_to_string("input/aoc2021_19")?;
        Ok(Self::parse_data(&data))
    }

    fn parse_data(data: &str) -> Self {
        let input = data
            .split("\n\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(Self::parse_point_set)
            .collect::<Vec<_>>();
        Self { input }
    }

    fn parse_point_set(data: &str) -> PointSet {
        data.split('\n')
            .map(|x| x.trim())
            .skip(1)
            .map(|x| HyperPoint::from_csv(x).expect("Invalid point format"))
            .inspect(|p| assert_eq!(p.dimension(), 3))
            .collect()
    }
}

impl Solution for AoC2021_19 {
    fn part_one(&self) -> String {
        let scanners = resolve_scanner_poles(&self.input);
        scanners
            .into_iter()
            .flat_map(|s| s.absolute_beacons())
            .collect::<HashSet<_>>()
            .len()
            .to_string()
    }

    fn part_two(&self) -> String {
        let scanners = resolve_scanner_poles(&self.input);
        let mut max_dist = 0;
        for (i, first) in scanners.iter().enumerate() {
            for second in scanners.iter().skip(i + 1) {
                let dist = first.pole.manhattan(&second.pole);
                max_dist = max_dist.max(dist);
            }
        }
        max_dist.to_string()
    }

    fn description(&self) -> String {
        "Day 19: Beacon Scanner".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2021_19_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2021_19_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "332");
        Ok(())
    }

    #[test]
    fn aoc2021_19_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "8507");
        Ok(())
    }

    #[test]
    fn aoc2021_19_case1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "79")
    }

    #[test]
    fn aoc2021_19_case2() {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "3621")
    }

    fn make_solution() -> io::Result<AoC2021_19> {
        AoC2021_19::new()
    }

    fn make_test_solution() -> AoC2021_19 {
        let data = "--- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401

        --- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390

        --- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562

        --- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596

        --- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14
";
        AoC2021_19::parse_data(data)
    }
}

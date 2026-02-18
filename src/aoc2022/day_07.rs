use crate::solution::Solution;
use crate::utils::*;

use std::collections::{HashMap, HashSet};
use std::io;

pub struct AoC2022_07 {
    input: Vec<String>,
}

impl AoC2022_07 {
    pub fn new() -> io::Result<Self> {
        let lines = read_file_as_lines("input/aoc2022_07")?;
        Ok(Self::parse_lines(&lines))
    }

    fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Self {
        let input = lines.iter().map(|x| x.as_ref().to_string()).collect();
        Self { input }
    }
}

impl Solution for AoC2022_07 {
    fn part_one(&self) -> String {
        let fs = make_fs(&self.input);
        let mut acc = 0;
        for route in fs.keys() {
            let total = folder_size(&fs, route);
            if total > 100000 {
                continue;
            }
            acc += total;
        }
        acc.to_string()
    }

    fn part_two(&self) -> String {
        let fs = make_fs(&self.input);
        const DISK_SIZE: usize = 70000000;
        const ENOUGH_SIZE: usize = 30000000;

        let used_size = folder_size(&fs, &Route::root());
        let free_space = DISK_SIZE - used_size;

        let mut smallest: Option<usize> = None;
        for route in fs.keys() {
            let size = folder_size(&fs, route);
            if free_space + size < ENOUGH_SIZE {
                continue;
            }
            smallest = smallest.map(|x| x.min(size)).or(Some(size));
        }

        smallest.map(|x| x.to_string()).unwrap_or(not_found())
    }

    fn description(&self) -> String {
        "Day 7: No Space Left On Device".to_string()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct FileInfo {
    name: String,
    size: usize,
}

#[derive(Debug, Default, Clone)]
struct FolderInfo {
    files: HashSet<FileInfo>,
    folders: HashSet<String>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Route {
    raw: Vec<String>,
}

impl Route {
    fn root() -> Self {
        Self {
            raw: vec!["/".to_string()],
        }
    }

    fn appending<T: AsRef<str>>(&self, elem: T) -> Self {
        let mut raw = self.raw.clone();
        raw.push(elem.as_ref().to_string());
        Self { raw }
    }
}

type FileSystem = HashMap<Route, FolderInfo>;

fn make_fs(input: &[String]) -> FileSystem {
    let mut stack = Vec::<Route>::new();
    stack.push(Route::root());
    let mut map = FileSystem::new();

    let mut expect_output = false;
    for line in input.iter() {
        if line.starts_with("$") {
            expect_output = false;
            let command = line.strip_prefix("$ ").expect("Invalid command");

            if command == "ls" {
                expect_output = true;
                continue;
            }

            if command.starts_with("cd") {
                let (_, path) = command
                    .split_once(' ')
                    .expect("cd without argument not allowed");

                match path {
                    "/" => stack = vec![Route::root()],
                    ".." => _ = stack.pop(),
                    _ => {
                        let route = stack.last().unwrap().appending(path);
                        stack.push(route);
                    }
                }
                continue;
            }

            unreachable!("unexpected command {command}");
        }

        if !expect_output {
            continue;
        }
        let (meta, name) = line.split_once(' ').expect("Invalid ls format");
        let path = stack.last().unwrap().clone();
        let entry = map.entry(path).or_default();
        if meta == "dir" {
            entry.folders.insert(name.to_string());
        } else {
            let size = meta.parse::<usize>().expect("Invalid size format");
            let info = FileInfo {
                name: name.to_string(),
                size,
            };
            entry.files.insert(info);
        }
    }

    map
}

fn folder_size(fs: &FileSystem, route: &Route) -> usize {
    let Some(info) = fs.get(route) else {
        return 0;
    };
    let mut total = info.files.iter().map(|x| x.size).sum::<usize>();

    for folder in info.folders.iter() {
        let r = route.appending(folder);
        total += folder_size(fs, &r);
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2022_07_input_load_test() -> io::Result<()> {
        let sol = make_solution()?;
        assert!(!sol.input.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2022_07_correctness_part_1() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_one(), "1989474");
        Ok(())
    }

    #[test]
    fn aoc2022_07_correctness_part_2() -> io::Result<()> {
        let sol = make_solution()?;
        assert_eq!(sol.part_two(), "1111607");
        Ok(())
    }

    fn make_solution() -> io::Result<AoC2022_07> {
        AoC2022_07::new()
    }

    #[test]
    fn aoc2022_07_case_1() {
        let sol = make_test_solution();
        assert_eq!(sol.part_one(), "95437");
    }

    #[test]
    fn aoc2022_07_case_2() {
        let sol = make_test_solution();
        assert_eq!(sol.part_two(), "24933642");
    }

    fn make_test_solution() -> AoC2022_07 {
        let input = [
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ];
        AoC2022_07::parse_lines(&input)
    }
}

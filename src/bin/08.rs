use adv_code_2024::*;
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{hash_map, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const EXAMPLE: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut antennas: HashMap<u8, Vec<Vector>> = HashMap::new();

        let lines = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| line.bytes().collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>();
        for (i, line) in lines.iter().enumerate() {
            for (j, &byte) in line.iter().enumerate() {
                if byte == b'.' {
                    continue;
                }
                let position = Vector {
                    x: i as i32,
                    y: j as i32,
                };
                antennas
                    .entry(byte)
                    .and_modify(|pos| pos.push(position))
                    .or_insert(vec![position]);
            }
        }
        let mut antinodes: HashSet<Vector> = HashSet::new();

        for (_, positions) in antennas {
            for combination in positions.iter().combinations(2) {
                let direction = *combination[0] - *combination[1];

                let a = *combination[0] + direction;
                let b = *combination[0] - direction;
                let c = *combination[1] + direction;
                let d = *combination[1] - direction;

                for antinode in [a, b, c, d] {
                    if !combination.contains(&&antinode) {
                        antinodes.insert(antinode);
                    }
                }
            }
        }

        Ok(antinodes
            .iter()
            .filter(|&pos| {
                pos.x >= 0
                    && pos.x < lines.len() as i32
                    && pos.y >= 0
                    && pos.y < lines[0].len() as i32
            })
            .count())
    }

    assert_eq!(14, part1(BufReader::new(EXAMPLE.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        Ok(0)
    }

    assert_eq!(34, part2(BufReader::new(EXAMPLE.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Vector {
    x: i32,
    y: i32,
}

impl Default for Vector {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

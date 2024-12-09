use adv_code_2024::*;
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const EXAMPLE: &str = "2333133121414131402";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut filesystem: Vec<Option<usize>> = Vec::new();
        let mut id: usize = 0;
        for (i, n) in reader
            .bytes()
            .map_while(Result::ok)
            .map(|b| b - 48)
            .enumerate()
        {
            if i % 2 == 0 {
                for _ in 0..n {
                    filesystem.push(Some(id));
                }
                id += 1
            } else {
                for _ in 0..n {
                    filesystem.push(None);
                }
            }
        }
        loop {
            let first_none = filesystem.iter().position(|x| x.is_none()).unwrap();
            let last_some = filesystem.iter().rposition(|x| x.is_some()).unwrap();
            if first_none < last_some {
                filesystem.swap(first_none, last_some);
            } else {
                break;
            }
        }
        Ok(filesystem
            .iter()
            .take_while(|&&x| x.is_some())
            .enumerate()
            .map(|(i, &x)| i * x.unwrap())
            .sum())
    }

    assert_eq!(1928, part1(BufReader::new(EXAMPLE.as_bytes()))?);

    // FIXME: Uncomment before committing
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part1(input_file)?);
    // println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut filesystem: Vec<Option<usize>> = Vec::new();
        let mut id: usize = 0;
        for (i, n) in reader
            .bytes()
            .map_while(Result::ok)
            .map(|b| b - 48)
            .enumerate()
        {
            if i % 2 == 0 {
                for _ in 0..n {
                    filesystem.push(Some(id));
                }
                id += 1
            } else {
                for _ in 0..n {
                    filesystem.push(None);
                }
            }
        }

        let chunks: Vec<Vec<Option<usize>>> = filesystem
            .into_iter()
            .chunk_by(|x| *x)
            .into_iter()
            .map(|(_, group)| group.collect())
            .collect();

        println!("{:?}", chunks);

        Ok(0)
    }

    assert_eq!(2858, part2(BufReader::new(EXAMPLE.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

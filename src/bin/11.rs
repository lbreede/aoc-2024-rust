use adv_code_2024::*;
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use rayon::prelude::*;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const EXAMPLE: &str = "0 1 10 99 999";
const EXAMPLE2: &str = "125 17";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R, iterations: usize) -> Result<usize> {
        let mut content = String::new();
        reader.read_line(&mut content)?;

        // Parse numbers into a VecDeque.
        let mut numbers: VecDeque<usize> = content
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("Should be a number"))
            .collect();

        let mut buffer = VecDeque::with_capacity(numbers.len() * 2);

        for i in 0..iterations {
            println!("Iteration {}", i + 1);

            buffer.clear(); // Reuse buffer to avoid reallocations.

            while let Some(number) = numbers.pop_front() {
                match number {
                    0 => buffer.push_back(1),
                    n if number_of_digits(n) % 2 == 0 => {
                        let div = 10usize.pow((number_of_digits(n) / 2) as u32);
                        buffer.push_back(n / div);
                        buffer.push_back(n % div);
                    }
                    n => buffer.push_back(n * 2024),
                }
            }

            // Swap numbers with buffer (no reallocation).
            std::mem::swap(&mut numbers, &mut buffer);
        }

        Ok(numbers.len())
    }

    assert_eq!(7, part1(BufReader::new(EXAMPLE.as_bytes()), 1)?);
    assert_eq!(22, part1(BufReader::new(EXAMPLE2.as_bytes()), 6)?);
    assert_eq!(55312, part1(BufReader::new(EXAMPLE2.as_bytes()), 25)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 25)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(EXAMPLE.as_bytes()))?);
    //
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 75)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn number_of_digits(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        (n as f64).log10() as usize + 1
    }
}

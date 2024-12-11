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

        // let mut buffer = VecDeque::with_capacity(numbers.len() * 2);

        for i in 0..iterations {
            println!("Iteration {}", i + 1);
            // Convert VecDeque into Vec for parallel processing.
            let numbers_vec: Vec<usize> = numbers.drain(..).collect();

            // Parallelize the processing of numbers using Rayon.
            let processed: Vec<usize> = numbers_vec
                .into_par_iter()
                .flat_map(|number| {
                    let digits = number_of_digits(number); // Precompute number of digits.
                    match number {
                        0 => vec![1], // Replace with 1.
                        _ if digits % 2 == 0 => {
                            let div = 10usize.pow((digits / 2) as u32);
                            vec![number / div, number % div] // Split number.
                        }
                        _ => vec![number * 2024], // Multiply by 2024.
                    }
                })
                .collect();

            // Convert processed Vec back to VecDeque for the next iteration.
            numbers = processed.into();
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

use adv_code_2024::*;
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const EXAMPLE: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut parse_page_nums = true;
        let mut page_ordering_rules: Vec<[usize; 2]> = Vec::new();
        let mut pages_to_produce: Vec<Vec<usize>> = Vec::new();
        for line in reader.lines().map_while(Result::ok) {
            if line.is_empty() {
                parse_page_nums = false;
                continue;
            }
            if parse_page_nums {
                let (x, y) = line.split_once("|").unwrap();
                let x: usize = x.parse()?;
                let y: usize = y.parse()?;
                page_ordering_rules.push([x, y]);
            } else {
                pages_to_produce.push(
                    line.split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect(),
                )
            }
        }

        let mut answer = 0;
        for pages in pages_to_produce {
            let mut k = 1; // FIXME: I HATE THIS
            'outer: for (i, &page) in pages.iter().enumerate() {
                for &next_page in pages[i + 1..].iter() {
                    if page_ordering_rules.contains(&[next_page, page]) {
                        k = 0;
                        break 'outer;
                    }
                }
            }
            answer += pages.get(pages.len().div_euclid(2)).unwrap() * k;
        }
        Ok(answer)
    }

    assert_eq!(143, part1(BufReader::new(EXAMPLE.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut parse_page_nums = true;
        let mut page_ordering_rules: Vec<[usize; 2]> = Vec::new();
        let mut pages_to_produce: Vec<Vec<usize>> = Vec::new();
        for line in reader.lines().map_while(Result::ok) {
            if line.is_empty() {
                parse_page_nums = false;
                continue;
            }
            if parse_page_nums {
                let (x, y) = line.split_once("|").unwrap();
                let x: usize = x.parse()?;
                let y: usize = y.parse()?;
                page_ordering_rules.push([x, y]);
            } else {
                pages_to_produce.push(
                    line.split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect(),
                )
            }
        }

        let mut answer = 0;
        for mut pages in pages_to_produce {
            let mut k = 0;

            // We loop over a range, rather than the vec itself since we want to mutate the vec
            // while looping over it.
            for i in 0..pages.len() - 1 {
                'inner: loop {
                    for j in (i + 1)..pages.len() {
                        // If `page_order_rules` contains the incorrect ordering, we swap the
                        // incorrect indices, set `k` to 1 to indicate that these pages were changed
                        // and finally `continue` the inner loop to re-check the ordering from the
                        // current `i`, not from the very beginning.
                        if page_ordering_rules.contains(&[pages[j], pages[i]]) {
                            pages.swap(i, j);
                            k = 1;
                            continue 'inner;
                        }
                        // If `j` reaches the end of pages, that means `i` is in the correct place
                        // We can therefore break out of the inner loop, increment `i` and continue
                        // looping.
                        if j == pages.len() - 1 {
                            break 'inner;
                        }
                    }
                }
            }
            answer += pages.get(pages.len().div_euclid(2)).unwrap() * k;
        }
        Ok(answer)
    }

    assert_eq!(123, part2(BufReader::new(EXAMPLE.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

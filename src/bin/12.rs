use adv_code_2024::*;
use anyhow::{Context, Result};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const EXAMPLE: &str = "\
AAAA
BBCD
BBCC
EEEC
";

const EXAMPLE2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

const EXAMPLE3: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

const EXAMPLE4: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

const EXAMPLE5: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let regions = find_regions(Grid::from(reader));

        // Calculate price
        let mut price: usize = 0;
        for region in regions {
            let area = region.len();
            let perimeter: usize = region
                .iter()
                .map(|pos| {
                    let x = pos.x;
                    let y = pos.y;
                    let mut neighbours = 0;
                    if region.iter().contains(&Position { x, y: y - 1 }) {
                        neighbours += 1;
                    }
                    if region.iter().contains(&Position { x: x + 1, y }) {
                        neighbours += 1;
                    }
                    if region.iter().contains(&Position { x, y: y + 1 }) {
                        neighbours += 1;
                    }
                    if region.iter().contains(&Position { x: x - 1, y }) {
                        neighbours += 1;
                    }
                    4 - neighbours
                })
                .sum();
            price += area * perimeter;
        }

        Ok(price)
    }

    assert_eq!(140, part1(BufReader::new(EXAMPLE.as_bytes()))?);
    assert_eq!(772, part1(BufReader::new(EXAMPLE2.as_bytes()))?);
    assert_eq!(1930, part1(BufReader::new(EXAMPLE3.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let regions = find_regions(Grid::from(reader));
        // Calculate price
        let mut price: usize = 0;
        for region in regions {
            let area = region.len();
            let sides: usize = region
                .iter()
                .map(|pos| {
                    let mut corners = 0;
                    let x = pos.x;
                    let y = pos.y;

                    let top = Position { x, y: y - 1 };
                    let right = Position { x: x + 1, y };
                    let bottom = Position { x, y: y + 1 };
                    let left = Position { x: x - 1, y };

                    // Top right
                    //
                    // +---+---+   +---+---+   +---+---+
                    // | A |   |   |   |   |   |   | A |
                    // +---X---+   +---X---+   +---X---+
                    // | A | A |   | A |   |   | A |   |
                    // +---+---+   +---+---+   +---+---+
                    //
                    let top_right = Position { x: x + 1, y: y - 1 };
                    if (region.contains(&top)
                        && region.contains(&right)
                        && !region.contains(&top_right))
                        || (!region.contains(&top)
                            && !region.contains(&right)
                            && !region.contains(&top_right))
                        || (!region.contains(&top)
                            && !region.contains(&right)
                            && region.contains(&top_right))
                    {
                        corners += 1;
                    }

                    // Bottom right
                    let bottom_right = Position { x: x + 1, y: y + 1 };
                    if (region.contains(&right)
                        && region.contains(&bottom)
                        && !region.contains(&bottom_right))
                        || (!region.contains(&right)
                            && !region.contains(&bottom)
                            && !region.contains(&bottom_right)
                            || (!region.contains(&right)
                                && !region.contains(&bottom)
                                && region.contains(&bottom_right)))
                    {
                        corners += 1;
                    }

                    // Bottom left
                    let bottom_left = Position { x: x - 1, y: y + 1 };
                    if (region.contains(&bottom)
                        && region.contains(&left)
                        && !region.contains(&bottom_left))
                        || (!region.contains(&bottom)
                            && !region.contains(&left)
                            && !region.contains(&bottom_left)
                            || (!region.contains(&bottom)
                                && !region.contains(&left)
                                && region.contains(&bottom_left)))
                    {
                        corners += 1;
                    }

                    // Top left
                    let top_left = Position { x: x - 1, y: y - 1 };
                    if (region.contains(&left)
                        && region.contains(&top)
                        && !region.contains(&top_left))
                        || (!region.contains(&left)
                            && !region.contains(&top)
                            && !region.contains(&top_left)
                            || (!region.contains(&left)
                                && !region.contains(&top)
                                && region.contains(&top_left)))
                    {
                        corners += 1;
                    }
                    corners
                })
                .sum();
            price += area * sides;
        }
        Ok(price)
    }

    assert_eq!(80, part2(BufReader::new(EXAMPLE.as_bytes()))?);
    assert_eq!(436, part2(BufReader::new(EXAMPLE2.as_bytes()))?);
    assert_eq!(236, part2(BufReader::new(EXAMPLE4.as_bytes()))?);
    assert_eq!(368, part2(BufReader::new(EXAMPLE5.as_bytes()))?);
    assert_eq!(1206, part2(BufReader::new(EXAMPLE3.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

fn flood_fill(target: u8, x: i32, y: i32, grid: &Grid, mut region: Vec<Position>) -> Vec<Position> {
    region.push(Position { x, y });
    if let Some(north) = grid.get_value(x, y - 1) {
        if north == target && !region.contains(&Position { x, y: y - 1 }) {
            region = flood_fill(target, x, y - 1, grid, region);
        }
    }
    if let Some(east) = grid.get_value(x + 1, y) {
        if east == target && !region.contains(&Position { x: x + 1, y }) {
            region = flood_fill(target, x + 1, y, grid, region);
        }
    }
    if let Some(south) = grid.get_value(x, y + 1) {
        if south == target && !region.contains(&Position { x, y: y + 1 }) {
            region = flood_fill(target, x, y + 1, grid, region);
        }
    }
    if let Some(west) = grid.get_value(x - 1, y) {
        if west == target && !region.contains(&Position { x: x - 1, y }) {
            region = flood_fill(target, x - 1, y, grid, region);
        }
    }
    region
}
fn find_regions(grid: Grid) -> Vec<Vec<Position>> {
    let mut regions: Vec<Vec<Position>> = Vec::new();
    for i in 0..grid.height() {
        for j in 0..grid.width() {
            let value = grid
                .get_value(j as i32, i as i32)
                .context("Invalid position")
                .unwrap();
            if !regions.iter().any(|region| {
                region.contains(&Position {
                    x: j as i32,
                    y: i as i32,
                })
            }) {
                let region: Vec<Position> = Vec::new();
                let region = flood_fill(value, j as i32, i as i32, &grid, region);
                regions.push(region);
            }
        }
    }
    regions
}

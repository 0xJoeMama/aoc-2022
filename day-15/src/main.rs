use std::collections::HashMap;

use aoc_codegen::day;
use aoc_lib::{parse, point::Point};

#[day(15, parser = parser)]
const DAY: u8 = 15;

fn parser(input: &str) -> HashMap<Point, u64> {
    input.lines().map(|l| {
        parse::regex!("sasd"; l => x, y);
    });
    todo!()
}

fn part1(input: &Vec<u32>) -> u32 {
    todo!()
}

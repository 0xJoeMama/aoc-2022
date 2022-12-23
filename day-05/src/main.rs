use std::{num::ParseIntError, str::FromStr};

use aoc_codegen::day;

#[day(5, parser = parser, part1 = part1, part2 = part2)]
const DAY: u8 = 5;

#[derive(Debug)]
struct Move {
    cnt: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let _ = iter.next();
        let cnt: usize = iter.next().unwrap().parse()?;
        let _ = iter.next();
        let from: usize = iter.next().unwrap().parse()?;
        let _ = iter.next();
        let to: usize = iter.next().unwrap().parse()?;
        Ok(Move {
            cnt,
            from: from - 1,
            to: to - 1,
        })
    }
}

fn parser(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let (init, moves) = input.split_once("\n\n").unwrap();
    let stacks_cnt = init.lines().last().unwrap().split_whitespace().count();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stacks_cnt];

    for line in init.lines().rev().skip(1) {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let package = line.chars().nth(4 * i + 1).unwrap();

            if !package.is_whitespace() {
                stack.push(package);
            }
        }
    }

    (
        stacks,
        moves.lines().flat_map(|l| l.parse::<Move>()).collect(),
    )
}

fn part1(input: &(Vec<Vec<char>>, Vec<Move>)) -> String {
    let mut stacks = input.0.clone();

    for mov in input.1.iter() {
        let from = &mut stacks[mov.from];
        let moved = from.split_off(from.len() - mov.cnt);
        let to = &mut stacks[mov.to];

        to.extend(moved.iter().rev());
    }

    stacks.iter().filter_map(|v| v.last()).collect::<String>()
}

fn part2(input: &(Vec<Vec<char>>, Vec<Move>)) -> String {
    let mut stacks = input.0.clone();

    for mov in input.1.iter() {
        let from = &mut stacks[mov.from];
        let moved = from.split_off(from.len() - mov.cnt);
        let to = &mut stacks[mov.to];

        to.extend_from_slice(&moved);
    }

    stacks.iter().filter_map(|v| v.last()).collect::<String>()
}

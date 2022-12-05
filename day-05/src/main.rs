use std::{num::ParseIntError, str::FromStr};

use aoc_lib::input;

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

fn main() {
    let _ = input::apply("input-day-05.txt", |f| {
        aoc_lib::timed(|| {
            let (init, moves) = f.split_once("\n\n").unwrap();
            let stacks_cnt = init.lines().last().unwrap().split_whitespace().count();
            let mut stacks: Vec<Vec<char>> = Vec::with_capacity(stacks_cnt);

            for _ in 0..stacks_cnt {
                stacks.push(Vec::new());
            }

            for line in init.lines().rev().skip(1) {
                for i in 0..stacks_cnt {
                    let package = line.chars().nth(4 * i + 1).unwrap();

                    if !package.is_whitespace() {
                        stacks[i].push(package);
                    }
                }
            }

            let moves = moves.lines().map(|l| l.parse::<Move>()).flatten().collect();

            aoc_lib::timed(|| println!("{}", part1(stacks.clone(), &moves)));
            aoc_lib::timed(|| println!("{}", part2(stacks, &moves)));
        });
    });
}

fn part1(mut stacks: Vec<Vec<char>>, moves: &Vec<Move>) -> String {
    for mov in moves {
        for _ in 0..mov.cnt {
            let from = &mut stacks[mov.from];
            let moved = from.pop().unwrap();
            let to = &mut stacks[mov.to];

            to.push(moved);
        }
    }

    stacks.iter().filter_map(|v| v.last()).collect::<String>()
}

fn part2(mut stacks: Vec<Vec<char>>, moves: &Vec<Move>) -> String {
    for mov in moves {
        let from = &mut stacks[mov.from];
        let moved = from.split_off(from.len() - mov.cnt);
        let to = &mut stacks[mov.to];

        to.extend_from_slice(&moved);
    }

    stacks.iter().filter_map(|v| v.last()).collect::<String>()
}

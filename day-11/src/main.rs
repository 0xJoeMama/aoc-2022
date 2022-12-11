use std::{collections::VecDeque, str::FromStr};

use aoc_lib::input;

#[derive(Debug, Clone)]
enum Op {
    Mul,
    Add,
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    inv: VecDeque<i64>,
    test: (i64, usize, usize),
    operation: (Op, i64),
    inspections: usize,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Monkey, Self::Err> {
        let mut lines = s.lines();
        lines.next();

        let inv = lines
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .split(", ")
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        let op_line = lines.next().unwrap();
        let mut op_iter = op_line.split_whitespace().skip(4);
        let op = op_iter.next().unwrap();

        let arg_str = op_iter.next().unwrap();
        let arg = arg_str.parse::<i64>().unwrap_or(0);

        let operation = match op {
            "*" => {
                if arg_str == "old" {
                    (Op::Square, 0)
                } else {
                    (Op::Mul, arg)
                }
            }
            "+" => (Op::Add, arg),
            other => unreachable!("Unknown operation: {}", other),
        };

        let test = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let try_target = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let else_target = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        Ok(Monkey {
            inv,
            test: (test, try_target, else_target),
            operation,
            inspections: 0,
        })
    }
}

impl Monkey {
    fn play(&mut self, mods: &[i64], relieved: bool) -> Vec<(usize, i64)> {
        for &modif in mods {
            self.inv.push_back(modif);
        }

        let mut out = Vec::new();
        while let Some(mut item) = self.inv.pop_front() {
            self.inspections += 1;

            let (op, arg) = &self.operation;
            item = match op {
                Op::Mul => item * arg,
                Op::Add => item + arg,
                Op::Square => item.pow(2),
            };

            if relieved {
                item /= 3;
            }

            let (div, target_true, target_false) = self.test;
            if item % div == 0 {
                out.push((target_true, item));
            } else {
                out.push((target_false, item));
            }
        }

        out
    }

    fn inspections(&self) -> usize {
        self.inspections
    }
}

fn main() {
    _ = input::apply("input-day-11.txt", |input| {
        let monkeys = input
            .split("\n\n")
            .map(|chunk| chunk.parse::<Monkey>().unwrap())
            .collect::<Vec<_>>();

        println!("Part 1: {}", aoc_lib::timed(|| part1(monkeys.clone())));
        println!("Part 2: {}", aoc_lib::timed(|| part2(monkeys)));
    });
}

fn part1(mut monkeys: Vec<Monkey>) -> usize {
    let mut modifications = Vec::with_capacity(monkeys.len());
    for _ in 0..monkeys.len() {
        modifications.push(Vec::new());
    }

    for _ in 0..20 {
        for (i, monkey) in monkeys.iter_mut().enumerate() {
            let out = monkey.play(&modifications[i], true);
            modifications[i].clear();

            for (target, item) in out {
                modifications[target].push(item);
            }
        }
    }

    monkeys.sort_by_key(Monkey::inspections);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(Monkey::inspections)
        .product()
}

fn part2(mut monkeys: Vec<Monkey>) -> usize {
    let mut modifications = Vec::with_capacity(monkeys.len());

    for _ in 0..monkeys.len() {
        modifications.push(Vec::new());
    }

    for _ in 0..10_000 {
        for (i, monkey) in monkeys.iter_mut().enumerate() {
            let out = monkey.play(&modifications[i], false);
            modifications[i].clear();

            for (target, item) in out {
                modifications[target].push(item);
            }
        }
    }

    println!(
        "Monkeh: {:#?}",
        monkeys.iter().map(Monkey::inspections).collect::<Vec<_>>()
    );

    monkeys.sort_by_key(Monkey::inspections);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(Monkey::inspections)
        .product()
}

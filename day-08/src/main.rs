use std::collections::HashMap;

use aoc_lib::{
    input,
    point::{Direction, Point},
};

// this solution is a bit weird because I forgot how I was planning on solving the problem, midway through.

fn main() {
    let _ = input::apply("input-day-08.txt", |input| {
        aoc_lib::timed(|| {
            let trees = input
                .lines()
                .enumerate()
                .flat_map(|(x, line)| {
                    line.chars().enumerate().map(move |(y, height)| {
                        (
                            Point::new(x as i64, y as i64, 0),
                            height.to_digit(10).unwrap(),
                        )
                    })
                })
                .collect::<HashMap<_, _>>();

            println!("{}", aoc_lib::timed(|| part1(&trees)));
            println!("{}", aoc_lib::timed(|| part2(&trees)));
        });
    });
}

fn part1(trees: &HashMap<Point, u32>) -> usize {
    trees
        .iter()
        .filter(|(p, init_height)| {
            Direction::plane()
                .iter()
                .map(Direction::to_point)
                .any(|ref unit| {
                    let mut p_moved = **p + *unit;

                    while let Some(&height) = trees.get(&p_moved) {
                        p_moved += unit;

                        if **init_height <= height {
                            return false;
                        }
                    }

                    true
                })
        })
        .count()
}

fn part2(trees: &HashMap<Point, u32>) -> usize {
    trees
        .iter()
        .map(|(p, init_height)| {
            Direction::plane()
                .iter()
                .map(Direction::to_point)
                .map(|ref unit| {
                    let mut view_distance = 0;
                    let mut p_moved = *p + unit;

                    while let Some(&height) = trees.get(&p_moved) {
                        view_distance += 1;
                        p_moved += unit;

                        if *init_height <= height {
                            return view_distance;
                        }
                    }

                    view_distance
                })
                .product::<usize>()
        })
        .max()
        .unwrap()
}

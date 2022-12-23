use aoc_codegen::day;
use aoc_lib::point::Point;
use std::collections::HashSet;
use std::str::FromStr;

use aoc_lib::point::Direction;

#[day(9, parser = parser, part1 = part1, part2 = part2)]
const DAY: u8 = 9;

#[derive(Debug)]
struct Move {
    dir: Direction,
    cnt: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, count) = s.split_once(' ').unwrap();
        let dir = match dir {
            "R" => Direction::PosX,
            "L" => Direction::NegX,
            "U" => Direction::PosY,
            "D" => Direction::NegY,
            _ => unreachable!(),
        };

        Ok(Move {
            dir,
            cnt: count.parse().unwrap(),
        })
    }
}

fn parser(input: &str) -> Vec<Move> {
    input.lines().map(|l| l.parse::<Move>().unwrap()).collect()
}

// allow me to explain:
// vector magic in this case allows this case to **not** follow the rules of the game
// specifically, for 1 tail we know that it will only move in the same column as the head, thus we can conclude this:
// suppose the head is at position (xh, yh) and the tail at (xt, yt)
// the head is moved by a unit vector in one direction that is always coordinal, thus the new head position either:
// (xh + 1, yh) or (xh - 1, yh) or (xh, yh + 1) or (xh, yh - 1), or more generalized (xh + dx, yh + dy) where either dx or dy is 0
//                                A
//                               /|
//                            C // B
// using the above example: A is the new head position B is the old one, C is the initial tail position
// we know CA = CB + BA, thus CB = CA - BA and CB is the movement we need to apply to the tail
fn part1(moves: &Vec<Move>) -> usize {
    let mut head_pos = *Point::origin();
    let mut tail_pos = *Point::origin();
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(tail_pos);

    for mov in moves {
        let unit = mov.dir.to_point();
        // vector magic
        for _ in 0..mov.cnt {
            head_pos += unit;

            if tail_pos.distance_squared(&head_pos) > 2 {
                tail_pos += head_pos - tail_pos - unit;
                visited.insert(tail_pos);
            }
        }
    }

    visited.len()
}

// sadly here I was forced to follow the rules bc aoc, man Im so sadge
fn part2(moves: &Vec<Move>) -> usize {
    let mut snaek = [*Point::origin(); 10];
    let mut visited = HashSet::new();
    visited.insert(*Point::origin());

    for mov in moves {
        let unit = mov.dir.to_point();
        for _ in 0..mov.cnt {
            snaek[0] += unit;

            for i in 0..9 {
                let head = snaek[i];
                let tail = &mut snaek[i + 1];

                if tail.distance_squared(&head) > 2 {
                    let mut distance_vector = head - *tail;

                    // sadly here we have to follow the rules :sadge:
                    if distance_vector.x == 0 || distance_vector.y == 0 {
                        distance_vector = distance_vector / 2;
                    } else {
                        distance_vector.x = distance_vector.x / distance_vector.x.abs();
                        distance_vector.y = distance_vector.y / distance_vector.y.abs();
                    }

                    *tail += distance_vector;
                }
            }

            visited.insert(snaek[9]);
        }
    }

    visited.len()
}

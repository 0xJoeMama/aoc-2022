use std::collections::HashSet;

use aoc_codegen::day;
use aoc_lib::point::Point;

#[day(14, parser = parser, part1 = part1, part2 = part2)]
const DAY: u8 = 14;

fn parser(input: &str) -> (i64, HashSet<Point>) {
    let mut res = HashSet::new();
    let mut highest_y = 0;
    for line in input.lines() {
        let mut iter = line.split("->").map(str::trim);
        let mut start_p: Point = iter
            .next()
            .unwrap()
            .split(',')
            .flat_map(str::parse)
            .collect();

        for curr_p in iter {
            let curr_p = curr_p.split(',').flat_map(str::parse).collect::<Point>();
            for p in start_p.points_between(&curr_p) {
                if p.y > highest_y {
                    highest_y = p.y;
                }

                res.insert(p);
            }

            start_p = curr_p;
        }
    }

    (highest_y, res)
}

const SOURCE: Point = Point::new(500, 0, 0);

fn find_next_move(p: &Point, locs: &HashSet<Point>) -> Option<Point> {
    let up = p.up();
    if !locs.contains(&up) {
        return Some(up);
    }

    let left = up.left();
    if !locs.contains(&left) {
        return Some(left);
    }

    let right = up.right();
    if !locs.contains(&right) {
        return Some(right);
    }

    None
}

// I understand there are performance hits because of the HashSet but I like the idea so I am going with it!
fn part1(input: &(i64, HashSet<Point>)) -> usize {
    let (highest_y, mut locations) = input.to_owned();
    let starting = locations.len();

    loop {
        let mut curr = SOURCE;

        loop {
            if curr.y > highest_y {
                return locations.len() - starting;
            }

            if let Some(next) = find_next_move(&curr, &locations) {
                curr = next;
            } else {
                locations.insert(curr);
                break;
            }
        }
    }
}

fn part2(input: &(i64, HashSet<Point>)) -> usize {
    let (highest_y, mut locations) = input.to_owned();
    let starting = locations.len();

    loop {
        let mut curr = SOURCE;

        loop {
            if curr.y == highest_y + 1 {
                locations.insert(curr);
                break;
            }

            let next = find_next_move(&curr, &locations);
            if let Some(next) = next {
                curr = next;
                continue;
            }

            locations.insert(curr);

            if curr == SOURCE {
                return locations.len() - starting;
            }

            break;
        }
    }
}

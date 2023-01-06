use aoc_codegen::day;
use aoc_lib::{
    point::Point,
    range::{Range, Relative},
};
use core::str::FromStr;
use std::convert::Infallible;

#[day(15, parser = parser, part1 = part1, part2 = part2)]
const DAY: u8 = 15;

struct Sensor {
    pos: Point,
    distance: i64,
}

impl FromStr for Sensor {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        aoc_lib::regex_parser! {
            "Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)";
            s => sensor_x, sensor_y, beacon_x, beacon_y
        };

        let pos = Point::new(sensor_x.parse().unwrap(), sensor_y.parse().unwrap(), 0);
        let beacon_pos = Point::new(beacon_x.parse().unwrap(), beacon_y.parse().unwrap(), 0);

        Ok(Self {
            pos,
            distance: pos.manhattan_distance(&beacon_pos),
        })
    }
}

fn parser(input: &str) -> Vec<Sensor> {
    let mut res: Vec<Sensor> = input.lines().flat_map(str::parse).collect();
    res.sort_by_key(|sensor| sensor.pos.x + sensor.distance);

    res
}

const TARGET_Y: i64 = 2_000_000;

fn find_line_poses(sensors: &[Sensor], target_y: i64) -> Vec<Range<i64>> {
    let mut ranges = sensors
        .iter()
        .filter_map(|s| {
            let h = (s.pos.y - target_y).abs();

            if h <= s.distance {
                let delta_x = s.distance - h;
                let x1 = s.pos.x - delta_x;
                let x2 = s.pos.x + delta_x;

                Some(Range::new(x1, x2))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    ranges.sort_by_key(Range::get_min);

    let mut merged = vec![ranges.remove(0)];
    for i in ranges {
        let last = merged.last_mut().unwrap();

        match i.relative(last) {
            Relative::Overlaps => {
                *last.mut_max() = i.get_max();
            }
            Relative::Disjoint => {
                merged.push(i);
            }
            Relative::Subrange => {}
        }
    }

    merged
}

fn part1(input: &[Sensor]) -> u64 {
    find_line_poses(input, TARGET_Y)
        .iter()
        .map(|r| r.get_span().unsigned_abs())
        .sum()
}

const MAX_COORD: i64 = 4_000_000;

fn part2(input: &[Sensor]) -> i64 {
    let beacon = (0..=MAX_COORD)
        .map(|y| find_line_poses(input, y))
        .enumerate()
        .find(|(_, r)| r.len() > 1)
        .map(|(y, ranges)| Point::new(ranges.first().unwrap().get_max() + 1, y as i64, 0))
        .unwrap();

    beacon.x * MAX_COORD + beacon.y
}

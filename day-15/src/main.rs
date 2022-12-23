use std::collections::HashMap;

use aoc_codegen::day;
use aoc_lib::point::Point;

#[day(15, parser = parser, part1 = part1, part2 = part2)]
const DAY: u8 = 15;

fn parser(input: &str) -> HashMap<Point, u64> {
    input
        .lines()
        .map(|line| {
            aoc_lib::regex_parse! {
                "Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)";
                line => sensor_x, sensor_y, beacon_x, beacon_y
            };

            let sensor = Point::new(sensor_x.parse().unwrap(), sensor_y.parse().unwrap(), 0);
            let beacon = Point::new(beacon_x.parse().unwrap(), beacon_y.parse().unwrap(), 0);
            let manhattan_distance = sensor.manhattan_distance(&beacon);

            (sensor, manhattan_distance)
        })
        .collect()
}

fn part1(_input: &HashMap<Point, u64>) -> u32 {
    todo!()
}

fn part2(_input: &HashMap<Point, u64>) -> u32 {
    todo!()
}

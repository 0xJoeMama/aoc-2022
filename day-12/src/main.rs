use std::collections::{HashMap, HashSet, VecDeque};

use aoc_lib::{
    input,
    point::{Direction, Point},
};

fn can_visit(start: char, end: char) -> bool {
    fn normalize(c: char) -> u8 {
        (match c {
            'S' => 'a',
            'E' => 'z',
            _ => c,
        } as u8)
    }

    let start = normalize(start);
    let end = normalize(end);

    if start > end {
        true
    } else {
        end - start <= 1
    }
}

fn bfs(start: &Point, end: &Point, map: &HashMap<Point, char>) -> Option<usize> {
    let mut queue = VecDeque::with_capacity(map.len());
    queue.push_back(start);
    let mut visited: HashSet<Point> = HashSet::new();
    let mut parent = HashMap::with_capacity(map.len());

    while let Some(current) = queue.pop_front() {
        let curr = map[current];

        for neighbor in Direction::plane().iter().map(|d| d.to_point() + current) {
            if let Some((nei_pos, val)) = map.get_key_value(&neighbor) {
                if !visited.contains(&neighbor) && can_visit(curr, *val) {
                    queue.push_back(nei_pos);
                    visited.insert(neighbor);
                    parent.insert(neighbor, current);
                }
            }
        }
    }

    let mut path_sz = 0;
    let mut current = end;

    while current != start {
        current = *parent.get(current)?;
        path_sz += 1;
    }

    Some(path_sz)
}

fn main() {
    _ = input::apply("input-day-12.txt", |input| {
        let points: HashMap<Point, char> = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(j, c)| (Point::new(i as i64, j as i64, 0), c))
            })
            .collect();

        let start = points.iter().find(|(_, c)| **c == 'S').unwrap().0;
        let end = points.iter().find(|(_, c)| **c == 'E').unwrap().0;

        println!(
            "Part 1: {}",
            aoc_lib::timed(|| part1(&points, start, end))
        );
        println!("Part 2: {}", aoc_lib::timed(|| part2(&points, end)));
    });
}

fn part1(points: &HashMap<Point, char>, start: &Point, end: &Point) -> usize {
    bfs(start, end, points).unwrap()
}

fn part2(points: &HashMap<Point, char>, end: &Point) -> usize {
    points
        .iter()
        .filter(|(_, c)| **c == 'a' || **c == 'S')
        .filter_map(|(p, _)| bfs(p, end, points))
        .min()
        .unwrap()
}

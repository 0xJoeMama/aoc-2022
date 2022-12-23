use std::collections::{HashMap, HashSet, VecDeque};

use aoc_codegen::day;
use aoc_lib::point::Point;

#[day(12, parser = parser, part1 = part1, part2 = part2)]
const DAY: u8 = 12;

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

        for neighbor in current.plane_neighbors() {
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

fn parser(input: &str) -> (HashMap<Point, char>, Point) {
    let points: HashMap<Point, char> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| (Point::new(i as i64, j as i64, 0), c))
        })
        .collect();
    let end = *points.iter().find(|(_, c)| **c == 'E').unwrap().0;

    (points, end)
}

fn part1(input: &(HashMap<Point, char>, Point)) -> usize {
    let (map, end) = input;
    let start = map.iter().find(|(_, c)| **c == 'S').unwrap().0;
    bfs(start, end, map).unwrap()
}

fn part2(input: &(HashMap<Point, char>, Point)) -> usize {
    let (map, end) = input;
    map.iter()
        .filter(|(_, c)| **c == 'a' || **c == 'S')
        .filter_map(|(p, _)| bfs(p, end, map))
        .min()
        .unwrap()
}

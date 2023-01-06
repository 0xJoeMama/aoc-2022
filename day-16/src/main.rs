use aoc_codegen::day;
use std::{collections::HashMap, convert::Infallible, str::FromStr};

#[day(16, parser = parser, part1 = part1, part2 = part2)]
const DAY: u8 = 16;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Node {
    name: String,
    pressure: usize,
}

#[allow(unused)]

struct Graph {
    nodes: Vec<Node>,
    index: HashMap<String, usize>,
    adj_list: Vec<Vec<(usize, usize)>>,
}

impl FromStr for Graph {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nodes: Vec<(Node, Vec<String>)> = s
        .lines()
        .map(|line| {
            aoc_lib::regex_parse! {
                "Valve ([A-Z]{2}) has flow rate=([0-9]+); tunnels? leads? to valves? ((?:[A-Z]{2},? ?)+)";
                line => name, pressure, children
            }

            (Node {
                name: name.to_string(),
                pressure: pressure.parse().unwrap(),
            }, children.split(',').map(str::trim).map(str::to_string).collect::<Vec<_>>())
        })
        .collect();

        let _name_to_idx: HashMap<String, usize> = nodes
            .iter()
            .enumerate()
            .map(|(i, n)| (n.0.name.clone(), i))
            .collect();
        todo!()
    }
}

fn parser(input: &str) -> Graph {
    input.parse().unwrap()
}

fn part1(_input: &Graph) -> usize {
    todo!()
}

fn part2(_input: &Graph) -> usize {
    todo!()
}

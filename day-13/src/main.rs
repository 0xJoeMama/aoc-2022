use aoc_lib::input;
use core::str::FromStr;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
enum PacketValue {
    List(Vec<PacketValue>),
    Number(i64),
}

impl Ord for PacketValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            PacketValue::Number(a) => match other {
                PacketValue::Number(b) => a.cmp(b),
                PacketValue::List(_) => self.to_list().unwrap().cmp(other),
            },
            PacketValue::List(a) => match other {
                PacketValue::List(b) => {
                    let mut a = a.iter();
                    let mut b = b.iter();
                    loop {
                        match (a.next(), b.next()) {
                            (Some(a), Some(b)) => match a.cmp(b) {
                                Ordering::Equal => continue,
                                x => return x,
                            },
                            (Some(_), None) => return Ordering::Greater,
                            (None, Some(_)) => return Ordering::Less,
                            (None, None) => return Ordering::Equal,
                        }
                    }
                }
                PacketValue::Number(_) => self.cmp(&other.to_list().unwrap()),
            },
        }
    }
}

impl PacketValue {
    fn to_list(&self) -> Option<PacketValue> {
        if let PacketValue::Number(val) = self {
            Some(PacketValue::List(vec![PacketValue::Number(*val)]))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Packet {
    value: PacketValue,
}

impl Packet {
    fn parse_list(iter: &mut impl Iterator<Item = char>) -> PacketValue {
        let mut list = Vec::new();

        let mut current = String::new();

        fn collect(container: &mut Vec<PacketValue>, current: &mut String) {
            if current.is_empty() {
                return;
            }

            container.push(PacketValue::Number(current.parse().unwrap()));
            current.clear();
        }

        while let Some(c) = iter.next() {
            match c {
                '[' => list.push(Packet::parse_list(iter)),
                ']' => {
                    collect(&mut list, &mut current);
                    break;
                }
                ',' => {
                    collect(&mut list, &mut current);
                }
                other => {
                    if other.is_numeric() {
                        current.push(other);
                    }
                }
            }
        }

        PacketValue::List(list)
    }
}

impl FromIterator<char> for Packet {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        iter.next();
        let res = Self::parse_list(&mut iter);
        Packet { value: res }
    }
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.chars().collect::<Packet>())
    }
}

fn main() {
    _ = input::apply("input-day-13.txt", |input| {
        aoc_lib::timed(|| {
            let input = input
                .split("\n\n")
                .map(|packets| {
                    let mut iter = packets.lines();
                    let packet1 = iter.next().unwrap().parse::<Packet>().unwrap();
                    let packet2 = iter.next().unwrap().parse::<Packet>().unwrap();
                    (packet1, packet2)
                })
                .collect::<Vec<_>>();

            println!("Part 1: {}", aoc_lib::timed(|| part1(&input)));
            println!("Part 2: {}", aoc_lib::timed(|| part2(input)));
        })
    });
}

fn part1(input: &[(Packet, Packet)]) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, (packet1, packet2))| packet1.value.cmp(&packet2.value) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(input: Vec<(Packet, Packet)>) -> usize {
    let mut packets: Vec<PacketValue> = input
        .into_iter()
        .flat_map(|(packet1, packet2)| [packet1.value, packet2.value])
        .collect();

    let div1 = PacketValue::List(vec![PacketValue::Number(6)]);
    let div2 = PacketValue::List(vec![PacketValue::Number(2)]);

    packets.push(div1.clone());
    packets.push(div2.clone());

    packets.sort_by(|a, b| a.cmp(b));

    let div1 = packets.iter().position(|p| *p == div1).unwrap() + 1;
    let div2 = packets.iter().position(|p| *p == div2).unwrap() + 1;
    div1 * div2
}

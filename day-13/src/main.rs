use aoc_codegen::day;
use core::str::FromStr;
use std::cmp::Ordering;

#[day(13, parser = parser, part1 = part1, part2 = part2)]
const DAY: u8 = 13;

#[derive(Debug, Eq, PartialEq, Clone)]
enum PacketValue {
    List(Vec<PacketValue>),
    Number(i64),
}

impl PartialOrd for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            PacketValue::Number(my_num) => match other {
                PacketValue::Number(other_num) => my_num.cmp(other_num),
                PacketValue::List(_) => self.to_list().cmp(other),
            },
            PacketValue::List(my_list) => match other {
                PacketValue::List(other_list) => {
                    let mut me = my_list.iter();
                    let mut other = other_list.iter();
                    loop {
                        match (me.next(), other.next()) {
                            (Some(mine), Some(his)) => match mine.cmp(his) {
                                Ordering::Equal => continue,
                                x => return x,
                            },
                            (Some(_), None) => return Ordering::Greater,
                            (None, Some(_)) => return Ordering::Less,
                            (None, None) => return Ordering::Equal,
                        }
                    }
                }
                PacketValue::Number(_) => self.cmp(&other.to_list()),
            },
        }
    }
}

impl PacketValue {
    fn to_list(&self) -> PacketValue {
        PacketValue::List(vec![self.clone()])
    }

    fn parse_list(iter: &mut impl Iterator<Item = char>) -> Self {
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
                '[' => list.push(Self::parse_list(iter)),
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

impl FromStr for PacketValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse_list(&mut s.chars().skip(1)))
    }
}

fn parser(input: &str) -> Vec<PacketValue> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .flat_map(|packet| packet.parse::<PacketValue>())
        .collect()
}

fn part1(packets: &[PacketValue]) -> usize {
    packets
        .chunks(2)
        .enumerate()
        .filter(|(_, packets)| packets[0] < packets[1])
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(packets: &[PacketValue]) -> usize {
    let mut packets = Vec::from(packets);

    let div1 = PacketValue::List(vec![PacketValue::Number(6)]);
    let div2 = PacketValue::List(vec![PacketValue::Number(2)]);

    packets.push(div1.clone());
    packets.push(div2.clone());

    packets.sort();

    let div1 = packets.iter().position(|p| *p == div1).unwrap() + 1;
    let div2 = packets.iter().position(|p| *p == div2).unwrap() + 1;
    div1 * div2
}

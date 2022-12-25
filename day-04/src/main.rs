use aoc_codegen::day;
use aoc_lib::range::NumberRange;

type Range = NumberRange<i32>;

#[day(4, parser = parser, part1 = part1, part2 = part2)]
const DAY: u8 = 4;

fn parser(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(',')
                .map(|pair| pair.parse::<Range>().unwrap())
                .collect::<(Range, Range)>()
        })
        .collect::<Vec<_>>()
}

fn part1(input: &[(Range, Range)]) -> usize {
    input
        .iter()
        .filter(|(first, second)| first.subrange(second) || second.subrange(first))
        .count()
}

fn part2(input: &[(Range, Range)]) -> usize {
    input
        .iter()
        .filter(|(first, second)| first.overlaps(second) || second.overlaps(first))
        .count()
}

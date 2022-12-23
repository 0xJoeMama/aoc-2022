use aoc_codegen::day;
use aoc_lib::range::NumberRange;

#[day(4, parser = parser, part1 = part1, part2 = part2)]
const DAY: u8 = 4;

fn parser(input: &str) -> Vec<(NumberRange, NumberRange)> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(',')
                .map(|pair| pair.parse().unwrap())
                .collect::<(NumberRange, NumberRange)>()
        })
        .collect::<Vec<_>>()
}

fn part1(input: &[(NumberRange, NumberRange)]) -> usize {
    input
        .iter()
        .filter(|(first, second)| first.subrange(second) || second.subrange(first))
        .count()
}

fn part2(input: &[(NumberRange, NumberRange)]) -> usize {
    input
        .iter()
        .filter(|(first, second)| first.overlaps(second) || second.overlaps(first))
        .count()
}

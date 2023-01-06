use aoc_codegen::day;
use aoc_lib::range::Range;

type Input = Vec<(Range<i32>, Range<i32>)>;

#[day(4, parser = parser, part1 = part1, part2 = part2)]
const DAY: u8 = 4;

fn parser(input: &str) -> Input {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(',')
                .flat_map(str::parse)
                .collect::<(Range<i32>, Range<i32>)>()
        })
        .collect::<Vec<_>>()
}

fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|(first, second)| first.subrange(second) || second.subrange(first))
        .count()
}

fn part2(input: &Input) -> usize {
    input
        .iter()
        .filter(|(first, second)| first.overlaps(second) || second.overlaps(first))
        .count()
}

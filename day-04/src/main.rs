use aoc_lib::input;
use aoc_lib::range::NumberRange;

fn main() {
    let _ = input::apply("input-day-04.txt", |f| {
        aoc_lib::timed(|| {
            let input = f
                .lines()
                .filter(|s| !s.is_empty())
                .map(|s| {
                    s.split(',')
                        .map(|pair| pair.parse().unwrap())
                        .collect::<(NumberRange, NumberRange)>()
                })
                .collect::<Vec<_>>();
            aoc_lib::timed(|| println!("{}", part1(&input)));
            aoc_lib::timed(|| println!("{}", part2(&input)));
        });
    });
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

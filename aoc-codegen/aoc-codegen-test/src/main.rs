use aoc_codegen::day;

#[day(1, parser = parse, part1 = part1, part2 = part2)]
const INPUT: u8 = 1;

fn parse(input: &str) -> u8 {
    input.parse().unwrap()
}

fn part1(input: &u8) -> u8 {
    *input
}

fn part2(input: &u8) -> u8 {
    *input
}

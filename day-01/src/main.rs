use aoc_codegen::day;

#[day(1, part1 = part1, part2 = part2)]
const DAY: u8 = 1;

fn part1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|three| {
            three
                .lines()
                .filter_map(|s| s.parse::<i32>().ok())
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> i32 {
    let mut calories = input
        .split("\n\n")
        .map(|three| {
            three
                .lines()
                .filter_map(|s| s.parse::<i32>().ok())
                .sum::<i32>()
        })
        .collect::<Vec<_>>();

    calories.sort_unstable();
    calories[calories.len() - 3..].iter().sum()
}

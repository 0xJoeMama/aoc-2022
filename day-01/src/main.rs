use aoc_lib::input;

fn main() {
    let _ = input::apply("input-day-01.txt", |f| {
        println!("{}", run_part1(f));
        println!("{}", run_part2(f));
    });
}

fn run_part1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|three| {
            three
                .lines()
                .filter_map(|s| s.parse::<i32>().ok())
                .sum::<i32>()
        })
        .max()
        .unwrap_or(-1)
}

fn run_part2(input: &str) -> i32 {
    let mut calories = input
        .split("\n\n")
        .map(|three| {
            three
                .lines()
                .filter_map(|s| s.parse::<i32>().ok())
                .sum::<i32>()
        })
        .collect::<Vec<_>>();

    calories.sort();
    calories[calories.len() - 3..].iter().sum()
}

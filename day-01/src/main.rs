use aoc_lib::input;

fn main() {
    let _ = input::apply("input.txt", |f| {
        println!("{}", run_part1(f));
        println!("{}", run_part2(f));
    });
}

fn run_part1(input: &String) -> i32 {
    input.split("\n\n")
        .map(|three| three.split("\n"))
        .map(|three| three.filter_map(|s| s.parse::<i32>().ok()).sum::<i32>())
        .max()
        .unwrap_or(-1)
}

fn run_part2(input: &String) -> i32 {
    let calories = input.split("\n\n")
        .map(|three| three.split("\n").filter_map(|s| s.parse::<i32>().ok()).sum::<i32>())
        .collect::<Vec<_>>();
    calories[calories.len() - 3..].iter().sum()
}

use std::collections::HashSet;

use aoc_lib::input;

fn calculate_score<T>(dups: T) -> i32
where
    T: IntoIterator<Item = char>,
{
    dups.into_iter().fold(0, |acc, c| {
        if c.is_lowercase() {
            acc + ((c as u8) - 96) as i32
        } else {
            acc + ((c as u8) - 65 + 27) as i32
        }
    })
}

fn main() {
    let _ = input::apply("input-day-03.txt", |file| {
        aoc_lib::timed(|| {
            let res: Vec<&str> = file.lines().filter(|l| !l.is_empty()).collect();
            aoc_lib::timed(|| {
                println!("{}", part1(&res));
            });

            aoc_lib::timed(|| {
                println!("{}", part2(&res));
            });
        });
    });
}

fn part1(input: &[&str]) -> i32 {
    input
        .iter()
        .map(|sack| {
            let (first, second) = sack.split_at(sack.len() / 2);
            let intersect = first.chars().filter(|c| second.contains(*c));
            calculate_score(intersect.collect::<HashSet<_>>())
        })
        .sum()
}

fn part2(input: &Vec<&str>) -> i32 {
    let mut badges = Vec::with_capacity(input.len() / 3);

    for i in 0..input.len() / 3 {
        let values = &input[i * 3..(i + 1) * 3];
        let badge = values[0]
            .chars()
            .filter(|c| values[1].contains(*c) && values[2].contains(*c))
            .last()
            .unwrap();

        badges.push(badge);
    }

    calculate_score(badges)
}

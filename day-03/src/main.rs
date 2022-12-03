use std::collections::HashSet;

use aoc_lib::input;

fn calculate_score<'a, T>(dups: T) -> i32
where
    T: Iterator<Item = &'a char>,
{
    let mut sum: i32 = 0;
    for c in dups {
        if c.is_lowercase() {
            sum += ((*c as u8) - 96) as i32;
        } else {
            sum += ((*c as u8) - 65 + 27) as i32;
        }
    }

    sum
}

fn main() {
    let _ = input::apply("input-day-03.txt", |file| {
        let res = file.split("\n").filter(|l| !l.is_empty()).collect();
        println!("{}", part1(&res));
        println!("{}", part2(&res));
    });
}

fn part1(input: &Vec<&str>) -> i32 {
    input
        .iter()
        .map(|sack| {
            let (first, second) = sack.split_at(sack.len() / 2);
            let unique = first.chars().collect::<HashSet<_>>();
            second
                .chars()
                .filter(|c| unique.contains(&c))
                .collect::<HashSet<_>>()
        })
        .map(|dups| calculate_score(dups.iter()))
        .sum()
}

fn part2(input: &Vec<&str>) -> i32 {
    let mut badges: Vec<char> = Vec::new();

    for i in 0..input.len() / 3 {
        let values = &input[i * 3..(i + 1) * 3];

        let badge = values[0]
            .chars()
            .filter(|c| values[1].contains(*c))
            .filter(|c| values[2].contains(*c))
            .last()
            .unwrap();

        badges.push(badge);
    }

    calculate_score(badges.iter())
}

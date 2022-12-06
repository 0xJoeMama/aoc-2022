use aoc_lib::input;

fn main() {
    let _ = input::apply("input-day-06.txt", |f| {
        aoc_lib::timed(|| {
            let input = f.split("\n").collect::<String>();
            aoc_lib::timed(|| println!("{}", part1(&input)));
            aoc_lib::timed(|| println!("{}", part2(&input)));
        });
    });
}

fn is_pure(s: &[u8]) -> bool {
    for (s_idx, s_byte) in s.iter().enumerate() {
        for (inner_idx, inner_byte) in s.iter().enumerate() {
            if s_idx != inner_idx && s_byte == inner_byte {
                return false;
            }
        }
    }

    true
}

fn part1(input: &str) -> usize {
    let out = input
        .as_bytes()
        .windows(4)
        .enumerate()
        .find(|(_, s)| is_pure(s))
        .unwrap()
        .0;

    out + 4
}

fn part2(input: &String) -> usize {
    let out = input
        .as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, s)| is_pure(s))
        .unwrap()
        .0;

    out + 14
}

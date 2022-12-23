use aoc_codegen::day;

#[day(6, parser = parser, part1 = part1, part2 = part2)]
const DAY: u8 = 6;

fn parser(input: &str) -> String {
    input.lines().collect()
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

use aoc_lib::input;

#[derive(PartialEq, Clone)]
#[repr(u8)]
enum State {
    Rock,
    Paper,
    Scissors,
}

impl State {
    fn get_winning(&self) -> Self {
        match self {
            State::Rock => State::Paper,
            State::Paper => State::Scissors,
            State::Scissors => State::Rock,
        }
    }

    fn get_losing(&self) -> Self {
        match self {
            State::Rock => State::Scissors,
            State::Paper => State::Rock,
            State::Scissors => State::Paper,
        }
    }

    fn get_score(&self) -> i32 {
        match self {
            State::Rock => 1,
            State::Paper => 2,
            State::Scissors => 3,
        }
    }
}

fn calculate_points(his: &State, mine: State) -> i32 {
    (if mine == *his {
        3
    } else if mine == his.get_winning() {
        6
    } else {
        0
    } + mine.get_score())
}

fn main() {
    let _ = input::apply("input-day-02.txt", |input| {
        aoc_lib::timed(|| {
            let input = aoc_lib::timed(|| {
                input
                    .lines()
                    .filter(|s| !s.is_empty())
                    .map(|line| {
                        let (his, mine) = line.split_at(1);
                        let his = match his {
                            "A" => State::Rock,
                            "B" => State::Paper,
                            "C" => State::Scissors,
                            _ => unreachable!("invalid input"),
                        };

                        (his, mine.trim_start())
                    })
                    .collect::<Vec<_>>()
            });

            aoc_lib::timed(|| part1(&input));
            aoc_lib::timed(|| part2(&input));
        });
    });
}

fn part1(input: &[(State, &str)]) {
    let res = input
        .iter()
        .map(|pair| {
            let mine = match pair.1 {
                "X" => State::Rock,
                "Y" => State::Paper,
                "Z" => State::Scissors,
                _ => unreachable!("invalid input"),
            };
            calculate_points(&pair.0, mine)
        })
        .sum::<i32>();
    println!("{res}");
}

fn part2(input: &[(State, &str)]) {
    let res = input
        .iter()
        .map(|pair| {
            let mine = match pair.1 {
                "X" => pair.0.get_losing(),
                "Y" => pair.0.clone(),
                "Z" => pair.0.get_winning(),
                _ => unreachable!("invalid input"),
            };
            calculate_points(&pair.0, mine)
        })
        .sum::<i32>();
    println!("{res}");
}

use std::str::Lines;

use aoc_lib::input;

fn main() {
    let _ = input::apply("input-day-10.txt", |input| {
        aoc_lib::timed(|| {
            aoc_lib::timed(|| println!("{}", solve(input)));
        });
    });
}

#[derive(Debug)]
struct Pc<'a> {
    instructions: Lines<'a>,
    clock: i64,
    halted: Option<i64>,
    x: i64,
    crt: u64,
    crt_pos: u8,
    draw_buffer: String,
}

impl<'a> Pc<'a> {
    fn new(inst: Lines<'_>) -> Pc<'_> {
        Pc {
            clock: 0,
            instructions: inst,
            halted: None,
            x: 1,
            crt: 0,
            crt_pos: 0,
            draw_buffer: String::with_capacity(41 * 6),
        }
    }

    fn poll_clock(&mut self) -> Option<i64> {
        self.clock += 1;
        if self.halted.is_some() {
            let res = self.get_result();
            self.draw();
            self.x += self.halted.take().unwrap();
            Some(res)
        } else if let Some(inst) = self.instructions.next() {
            let mut iter = inst.split_whitespace();

            if iter.next().unwrap() == "addx" {
                self.halted = iter.next().unwrap().parse().ok();
            }

            self.draw();
            Some(self.get_result())
        } else {
            self.draw();

            print!("{}", self.draw_buffer);
            None
        }
    }

    fn draw(&mut self) {
        if self.crt_pos == 40 {
            for i in 0..40 {
                let current_pixel = self.crt & (1 << i);

                if current_pixel != 0 {
                    self.draw_buffer.push('█');
                } else {
                    self.draw_buffer.push(' ');
                }
            }

            self.draw_buffer.push('\n');

            self.crt_pos = 0;
            self.crt = 0;
        }

        if (self.crt_pos as i8 - self.x as i8).abs() <= 1 {
            self.crt |= 1 << self.crt_pos;
        }

        self.crt_pos += 1;
    }

    fn get_result(&self) -> i64 {
        if (self.clock - 20) % 40 == 0 {
            self.clock * self.x
        } else {
            0
        }
    }
}

fn solve(input: String) -> i64 {
    let mut sum = 0;
    let mut pc = Pc::new(input.lines());

    while let Some(amount) = pc.poll_clock() {
        sum += amount;
    }

    sum
}

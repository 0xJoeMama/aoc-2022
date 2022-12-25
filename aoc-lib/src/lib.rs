use std::time::Instant;

pub mod input;
pub mod parse;
pub mod point;
pub mod range;

pub mod either;

pub fn timed<T>(f: impl FnOnce() -> T) -> T {
    let timer = Instant::now();
    let res = f();
    println!("Task took {:?} to finish!", Instant::now() - timer);
    res
}

#[macro_export]
macro_rules! timer {
    ($($arg:tt)*) => {{
        let timer = std::time::Instant::now();

        let res = {
            $($arg)*
        };

        println!("Timer took {:?} to finish!", timer.elapsed());

        res
    }}
}

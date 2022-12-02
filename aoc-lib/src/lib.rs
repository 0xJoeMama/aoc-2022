use std::time::Instant;

pub mod input;

pub fn timed<T>(f: impl FnOnce() -> T) -> T {
    let timer = Instant::now();
    let res = f();
    println!("Task took {:?} to finish!", Instant::now() - timer);
    res
}

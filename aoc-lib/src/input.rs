use std::io::Result;

pub fn apply(f: impl FnOnce(String)) -> Result<()> {
    let file = std::fs::read_to_string("")?;
    f(file);
    return Ok(());
}

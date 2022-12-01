use std::io::Result;

pub fn apply(file: &str, f: impl FnOnce(&String)) -> Result<()> {
    let file = std::fs::read_to_string(file)?;
    f(&file);
    return Ok(());
}

use std::io::Result;

pub fn apply(file: &str, function: impl FnOnce(&String)) -> Result<()> {
    let file = std::fs::read_to_string(file)?;
    function(&file);
    Ok(())
}

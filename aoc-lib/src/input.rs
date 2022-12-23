use std::fs;
use std::io::Result;

pub fn apply(path: &str, function: impl FnOnce(String)) -> Result<()> {
    let file = fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read {path}"));
    function(file);
    Ok(())
}

use std::fs;
use std::io::Result;

/// # Errors
///
/// Will return an error if the file cannot be read.
pub fn apply(path: &str, function: impl FnOnce(String)) -> Result<()> {
    let file = fs::read_to_string(path)?;
    function(file);
    Ok(())
}

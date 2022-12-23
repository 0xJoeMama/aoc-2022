use std::io::Result;

pub fn apply(path: &str, function: impl FnOnce(String)) -> Result<()> {
    let file = std::fs::read_to_string(path).expect(format!("Failed to read {}", path).as_str());
    function(file);
    Ok(())
}

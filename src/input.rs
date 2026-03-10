
use crate::error::RshError;
use std::io::{self, Write};

// Prints prompt before reading one line from stdin.
pub fn read_line(prompt: &str) -> Result<Option<String>, RshError> {
    print!("{prompt}");
    io::stdout().flush().map_err(RshError::FlushFailed)?;

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => Ok(None),
        Ok(_) => Ok(Some(input)),
        Err(e) => Err(RshError::ReadFailed(e)),
    }
}

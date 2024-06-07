use std::fs::read_to_string;

use anyhow::Result;

pub fn get_token() -> Result<String> {
    Ok(read_to_string("./secrets/token.txt")?)
}

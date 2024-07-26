use std::{fs::read_to_string, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::utils::json::{from_json, write_json};

pub fn get_token() -> Result<String> {
    Ok(read_to_string("./secrets/token.txt")?)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(rename = "cmdPrefix")]
    #[serde(default)]
    pub cmd_prefix: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cmd_prefix: String::from("!"),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        if !Path::new("./config/config.json").exists() {
            let defcfg = Config::default();
            write_json("./config/config.json", &defcfg).unwrap();
            defcfg
        } else {
            from_json::<Self>("./config/config.json").unwrap()
        }
    }
}

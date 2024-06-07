use std::{
    fs::{read, DirBuilder, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_slice, json, ser::PrettyFormatter, Serializer, Value};

pub fn read_json<P: AsRef<Path>>(path: P) -> Result<Value> {
    let val = match read(&path) {
        Ok(file) => file,
        Err(_) => {
            let dir = PathBuf::from(path.as_ref());
            let dir = dir.parent().unwrap();
            DirBuilder::new().recursive(true).create(dir).unwrap();
            return Ok(json!({}));
        }
    };
    match from_slice(&val) {
        Ok(value) => Ok(value),
        Err(_) => Ok(json!({})),
    }
}

pub fn from_json<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let val = read_json(path)?;
    let data = T::deserialize(val)?;
    Ok(data)
}

pub fn write_json<T: Serialize>(path: &str, value: T) -> Result<()> {
    let mut file = if !PathBuf::from(path).exists() {
        let dir = PathBuf::from(path);
        let dir = dir.parent().unwrap();
        DirBuilder::new().recursive(true).create(dir).unwrap_or(());
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?
    } else {
        OpenOptions::new().write(true).truncate(true).open(path)?
    };
    let fmt = PrettyFormatter::with_indent(b"    ");
    let mut buf = Vec::new();
    let mut ser = Serializer::with_formatter(&mut buf, fmt);
    value.serialize(&mut ser)?;
    Ok(file.write_all(&buf)?)
}

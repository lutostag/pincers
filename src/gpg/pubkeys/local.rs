use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn matches(url: &str) -> bool {
    url.starts_with("file:") && Path::new(&url[7..]).exists() || Path::new(&url).exists()
}

pub fn read(url: &str) -> Result<Box<dyn BufRead>> {
    Ok(Box::new(BufReader::new(File::open(url)?)))
}

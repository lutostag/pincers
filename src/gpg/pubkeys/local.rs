use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn remove_prefix(url: &str) -> &str {
    if url.starts_with("file://") {
        &url[7..]
    } else {
        url
    }
}

pub fn matches(url: &str) -> bool {
    let url = remove_prefix(url);
    Path::new(&url).exists()
}

pub fn read(url: &str) -> Result<Box<dyn BufRead>> {
    let url = remove_prefix(url);
    Ok(Box::new(BufReader::new(File::open(url)?)))
}

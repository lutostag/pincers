use anyhow::Result;
use reqwest;
use std::io::{BufRead, Cursor, Read};

pub fn matches(url: &str) -> bool {
    url.starts_with("https:") || url.starts_with("http:")
}

pub fn read(url: &str) -> Result<Box<dyn BufRead>> {
    let mut body = Vec::<u8>::new();
    reqwest::get(url)?
        .error_for_status()?
        .read_to_end(&mut body)?;
    Ok(Box::new(Cursor::new(body)))
}

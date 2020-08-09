use super::http;
use failure::Error;
use std::io::BufRead;

pub fn matches(url: &str) -> bool {
    url.starts_with("keybase:")
}

pub fn read(url: &str) -> Result<Box<dyn BufRead>, Error> {
    let url = format!("https://keybase.io/{}/pgp_keys.asc", &url[8..]);
    Ok(Box::new(http::read(&url)?))
}

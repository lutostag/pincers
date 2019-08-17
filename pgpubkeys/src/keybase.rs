use failure::Error;
use reqwest;
use std::io::Read;

pub fn matches(url: &str) -> bool {
    url.starts_with("kb:")
}

pub fn download(url: &str) -> Result<Vec<u8>, Error> {
    let mut body = Vec::<u8>::new();
    let url: String = format!("https://keybase.io/{}/pgp_keys.asc", &url[3..]);
    reqwest::get(&url)?
        .error_for_status()?
        .read_to_end(&mut body)?;
    Ok(body)
}

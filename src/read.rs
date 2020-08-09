use failure::Error;
use reqwest;

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};

lazy_static! {
    static ref REMOTE_SCHEMES: HashSet<&'static str> = hashset! {
        "https", "http", "ftps", "ftp"
    };
}

pub fn is_remote(url: &str) -> bool {
    if let Ok(url) = reqwest::Url::parse(url) {
        return REMOTE_SCHEMES.contains(&url.scheme());
    }
    false
}

pub fn download(url: &str) -> Result<Vec<u8>, Error> {
    info!("Getting script: {}", url);
    let mut body = Vec::<u8>::new();

    if url == "-" {
        debug!("Trying to read from stdin");
        io::stdin().read_to_end(&mut body)?;
    } else if is_remote(&url) {
        debug!("Trying to read from remote {}", &url);
        reqwest::get(url)?
            .error_for_status()?
            .read_to_end(&mut body)?;
    } else {
        debug!("Trying to read from local {}", &url);
        File::open(url)?.read_to_end(&mut body)?;
    };
    match std::str::from_utf8(&body) {
        Ok(val) => debug!("Read contents\n{}", val),
        Err(_) => debug!("Read content is binary"),
    }
    Ok(body)
}

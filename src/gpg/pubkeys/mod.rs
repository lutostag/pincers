use failure::Error;
use gpgrv::Keyring;
use std::io::BufRead;

mod http;
mod keybase;
mod local;

fn add_key(keyring: &mut Keyring, mut from: Box<dyn BufRead>) -> Result<(), Error> {
    let first_byte = {
        let head = from.fill_buf()?;
        ensure!(!head.is_empty(), "empty file");
        head[0]
    };

    match first_byte {
        b'-' => keyring.append_keys_from_armoured(from)?,
        _ => keyring.append_keys_from(from)?,
    };
    Ok(())
}

pub fn add_key_from_url(keyring: &mut Keyring, url: &str) -> Result<(), Error> {
    let data = match url {
        _ if http::matches(url) => http::read(url)?,
        _ if keybase::matches(url) => keybase::read(url)?,
        _ if local::matches(url) => local::read(url)?,
        _ => bail!("No way to read gpg key from {}", url),
    };
    add_key(keyring, data)
}

#[test]
fn it_works() {
    let mut keyring = Keyring::new();
    add_key_from_url(&mut keyring, "keybase:lutostag").unwrap();
    assert_eq!(2 + 2, 4);
}

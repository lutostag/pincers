use failure::Error;
use gpgrv;
use std::io;

mod pubkeys;

pub fn verify(input_file: &[u8], gpg_key_url: &str) -> Result<(), Error> {
    let mut keyring = gpgrv::Keyring::new();
    pubkeys::add_key_from_url(&mut keyring, gpg_key_url)?;
    gpgrv::verify_message(io::Cursor::new(input_file), vec![], &keyring)
}

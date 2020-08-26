use anyhow::Result;
use gpgrv::Keyring;

pub use gpgrv::{verify_detached, verify_message};

mod pubkeys;

pub fn create_keyring(gpg_key_url: &str) -> Result<Keyring> {
    let mut keyring = Keyring::new();
    pubkeys::add_key_from_url(&mut keyring, gpg_key_url)?;
    Ok(keyring)
}

// pub fn verify_inline(input_file: &[u8], keyring: &Keyring) -> Result<(), Error> {
//     gpgrv::verify_message(io::Cursor::new(input_file), vec![], keyring)
// }
//
// pub fn verify_detached(
//     signature: &[u8],
//     input_file: &[u8],
//     keyring: &Keyring,
// ) -> Result<(), Error> {
//     gpgrv::verify_detached(
//         io::Cursor::new(signature),
//         io::Cursor::new(input_file),
//         keyring,
//     )
// }

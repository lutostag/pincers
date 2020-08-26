use crate::read::download;
use anyhow::{Context, Result};
use gpgrv::{verify_detached, verify_message, Keyring};
use std::io::Cursor;
use std::mem::replace;

mod pubkeys;

pub fn create_keyring(gpg_key_url: &str) -> Result<Keyring> {
    let mut keyring = Keyring::new();
    pubkeys::add_key_from_url(&mut keyring, gpg_key_url)?;
    Ok(keyring)
}

pub fn verify(url: &str, data: &mut Vec<u8>, keyring: &Keyring) -> Result<()> {
    let mut new_data = vec![];
    if verify_message(Cursor::new(&data), &mut new_data, &keyring).is_ok() {
        let _ = replace(data, new_data); // if inline have to remove the signature itself
    } else if let Ok(sig) = download(&format!("{}.sig", url)) {
        info!("Found .sig file to match, attempting to verify signature");
        verify_detached(Cursor::new(&sig), Cursor::new(&data), &keyring)
            .context("Found .sig signature with invalid signature data")?;
    } else if let Ok(asc) = download(&format!("{}.asc", url)) {
        info!("Found .asc file to match, attempting to verify signature");
        verify_detached(Cursor::new(&asc), Cursor::new(&data), &keyring)
            .context("Found .asc signature with invalid signature data")?;
    } else {
        bail!("Signature not verified")
    }
    Ok(())
}

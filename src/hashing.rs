use anyhow::Result;
use digest::DynDigest;
use md5;
use sha1;
use sha2;
use sha3;

#[derive(Debug, Clone, Copy)]
pub enum HashType {
    MD5,  // bits: 128
    SHA1, // bits: 160
    SHA2, // bits: 224, 256, 384, 512
    SHA3, // bits: 224, 256, 384, 512
}

pub fn hex_bits(sum: &str) -> Result<u16> {
    if let Some(c) = sum.chars().find(|c| !c.is_digit(16)) {
        bail!("Non hexdigit {} found in checksum", c)
    } else {
        Ok(sum.len() as u16 * 4)
    }
}

pub fn digest(hash: HashType, bits: Option<u16>) -> Result<Box<dyn DynDigest>> {
    match (hash, bits) {
        (HashType::MD5, Some(128)) => Ok(Box::new(md5::Md5::default())),
        (HashType::MD5, None) => Ok(Box::new(md5::Md5::default())),
        (HashType::SHA1, Some(160)) => Ok(Box::new(sha1::Sha1::default())),
        (HashType::SHA1, None) => Ok(Box::new(sha1::Sha1::default())),
        (HashType::SHA2, Some(224)) => Ok(Box::new(sha2::Sha224::default())),
        (HashType::SHA2, Some(256)) => Ok(Box::new(sha2::Sha256::default())),
        (HashType::SHA2, Some(384)) => Ok(Box::new(sha2::Sha384::default())),
        (HashType::SHA2, Some(512)) => Ok(Box::new(sha2::Sha512::default())),
        (HashType::SHA2, None) => Ok(Box::new(sha2::Sha512::default())),
        (HashType::SHA3, Some(224)) => Ok(Box::new(sha3::Sha3_224::default())),
        (HashType::SHA3, Some(256)) => Ok(Box::new(sha3::Sha3_256::default())),
        (HashType::SHA3, Some(384)) => Ok(Box::new(sha3::Sha3_384::default())),
        (HashType::SHA3, Some(512)) => Ok(Box::new(sha3::Sha3_512::default())),
        (HashType::SHA3, None) => Ok(Box::new(sha3::Sha3_512::default())),
        _ => bail!(
            "{:?} with length {} ({} bits) is invalid; double check entered checksum",
            &hash,
            bits.unwrap() / 4,
            bits.unwrap()
        ),
    }
}

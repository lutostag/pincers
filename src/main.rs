#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate digest;
extern crate hex;
extern crate md5;
extern crate reqwest;
extern crate sha1;
extern crate sha2;
extern crate sha3;
extern crate simplelog;

mod cli;
mod hashing;

use clap::ArgMatches;
use digest::DynDigest;
use failure::Error;
use hashing::{digest, HashType};
use hex::encode;
use log::Level;
use simplelog::{Config, LevelFilter, TermLogger};
use std::io::{Read, Write};
use std::process::{Command, Stdio};

fn hash_instance(args: &ArgMatches) -> Result<Box<DynDigest>, Error> {
    let hash = if args.is_present("md5") {
        warn!(
            "MD5 is not considered secure, please use a more secure checksum algorithm if possible"
        );
        Some(HashType::MD5)
    } else if args.is_present("sha1") {
        warn!("SHA-1 is not considered secure, please use a more secure checksum algorithm if possible");
        Some(HashType::SHA1)
    } else if args.is_present("sha2") {
        Some(HashType::SHA2)
    } else if args.is_present("sha3") {
        Some(HashType::SHA3)
    } else {
        None
    };
    if let Some(sum) = args.value_of("hash") {
        digest(hash.unwrap(), &sum)
    } else {
        bail!("No checksum provided to verify against")
    }
}

fn download(url: &str) -> Result<Vec<u8>, Error> {
    info!("Getting script: {}", url);
    let mut body = Vec::<u8>::new();
    reqwest::get(url)?
        .error_for_status()?
        .read_to_end(&mut body)?;
    debug!(
        "Downloaded script contents\n{}",
        std::str::from_utf8(&body)?
    );
    Ok(body)
}

fn check_hash(mut digest: Box<DynDigest>, body: &Vec<u8>, known: &str) -> bool {
    digest.input(body);
    let calculated = encode(digest.result());
    info!("Hash of downloaded script: {}", calculated);
    calculated.eq_ignore_ascii_case(known)
}

fn exec(command: &str, downloaded: &Vec<u8>) -> Result<(), Error> {
    info!("Starting command '{}'", command);
    let mut child = Command::new(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn()?;

    debug!("Piping in script contents to command");
    let stdin = child.stdin.as_mut().expect("Cannot open stdin");
    stdin.write_all(downloaded)?;
    Ok(())
}

fn setup_logging(verbosity: u64) -> Result<(), simplelog::TermLogError> {
    let mut config = Config::default();
    config.time = Some(Level::Debug);
    let log_level = match verbosity {
        0 => LevelFilter::Error,
        1 => LevelFilter::Info, // Includes Warn as well
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    TermLogger::init(log_level, config)
}

fn run() -> Result<(), Error> {
    let args = cli::args();
    setup_logging(args.occurrences_of("v"))?;
    let digest = hash_instance(&args)?;
    let checksum = args.value_of("hash").unwrap();
    if let Some(url) = args.value_of("URL") {
        let data = download(&url)?;
        if check_hash(digest, &data, &checksum) {
            info!("Checksum matches");
            exec("sh", &data)?;
        } else {
            bail!("Checksum does not match");
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        error!("{}", err);
        std::process::exit(1);
    }
}

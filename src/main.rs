#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;

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
mod read;

use digest::DynDigest;
use failure::Error;
use hashing::{digest, hex_bits, HashType};
use hex::encode;
use log::Level;
use read::download;
use simplelog::{Config, LevelFilter, TermLogger};
use std::io::Write;
use std::process::{Command, ExitStatus, Stdio};

fn hash_instance(algo: &str, sum: &Option<&str>) -> Result<Box<DynDigest>, Error> {
    let algo = match algo {
        "MD5" => {
            warn!("MD5 is not considered secure, please use a more secure checksum algorithm if possible");
            HashType::MD5
        }
        "SHA1" => {
            warn!("SHA-1 is not considered secure, please use a more secure checksum algorithm if possible");
            HashType::SHA1
        }
        "SHA2" => HashType::SHA2,
        "SHA3" => HashType::SHA3,
        _ => bail!("No checksum provided to verify against"),
    };
    let bits = match sum {
        Some(x) => Some(hex_bits(x)?),
        None => None,
    };
    digest(algo, bits)
}

fn compute_hash(mut digest: Box<DynDigest>, body: &[u8]) -> String {
    digest.input(body);
    let computed = encode(digest.result());
    info!("Hash: {}", &computed);
    computed
}

fn hash_matches(computed: &str, expected: &str) -> bool {
    computed.eq_ignore_ascii_case(expected)
}

fn exec(command: &str, downloaded: &[u8]) -> Result<ExitStatus, Error> {
    info!("Starting command '{}'", command);
    let mut child = Command::new(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn()?;

    {
        debug!("Piping in contents to command");
        let stdin = child.stdin.as_mut().expect("Cannot open stdin");
        stdin.write_all(downloaded)?;
    }

    let status = child.wait()?;
    debug!("Finished running with exit status: {}", status);
    Ok(status)
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

fn run(args: clap::ArgMatches) -> Result<Option<ExitStatus>, Error> {
    setup_logging(args.occurrences_of("v"))?;
    if let (command, Some(args)) = args.subcommand() {
        let checksum = args.value_of("HASH");
        let url = args.value_of("URL");
        let algo = args.value_of("ALGO");
        let digest = hash_instance(&algo.unwrap(), &checksum)?;
        if url.is_none() {
            bail!("No URL/filename given")
        }
        let data = download(&url.unwrap())?;
        let computed = compute_hash(digest, &data);
        match command {
            "hash" => {
                println!("{} {} {}", &url.unwrap(), &algo.unwrap(), &computed);
                return Ok(None);
            }
            "verify" if hash_matches(&computed, &checksum.unwrap()) => {
                info!("Checksum matches");
                return Ok(None);
            }
            "run" if hash_matches(&computed, &checksum.unwrap()) => {
                info!("Checksum matches");
                return Ok(Some(exec("sh", &data)?));
            }
            _ => {
                bail!("Checksum does not match");
            }
        }
    } else {
        bail!("No valid subcommand given");
    }
}

fn main() {
    let status = run(cli::args().get_matches());
    let exit_code = match status {
        Err(err) => {
            error!("{}", err);
            1
        }
        Ok(Some(exit_status)) => exit_status.code().unwrap_or(1),
        Ok(None) => 0,
    };
    std::process::exit(exit_code)
}

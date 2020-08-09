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
extern crate gpgrv;
extern crate hex;
extern crate md5;
extern crate reqwest;
extern crate sha1;
extern crate sha2;
extern crate sha3;
extern crate simplelog;
extern crate uuid;
use uuid::Uuid;

mod cli;
mod gpg;
mod hashing;
mod read;

use digest::DynDigest;
use failure::Error;
use hashing::{digest, hex_bits, HashType};
use hex::encode;
use log::Level;
use read::download;
use simplelog::{Config, LevelFilter, TermLogger};
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::{Command, ExitStatus, Stdio};

fn hash_instance(algo: &str, sum: &Option<&str>) -> Result<Box<dyn DynDigest>, Error> {
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

fn compute_hash(mut digest: Box<dyn DynDigest>, body: &[u8]) -> String {
    digest.input(body);
    let computed = encode(digest.result());
    info!("Hash: {}", &computed);
    computed
}

fn hash_matches(computed: &str, expected: &str) -> bool {
    computed.eq_ignore_ascii_case(expected)
}

fn exec(command: &str, args: Vec<&str>, downloaded: &[u8]) -> Result<ExitStatus, Error> {
    info!("Starting command '{}' with args {:?}", command, args);
    let mut child = Command::new(command)
        .args(args)
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
    if let (command, Some(args)) = args.subcommand() {
        let url = args.value_of("URL");
        let algo = args.value_of("ALGO");
        let checksum = args.value_of("HASH");
        if url.is_none() {
            bail!("No URL/filename given")
        }
        let data = download(&url.unwrap())?;

        match command {
            "hash" => {
                if algo == Some("GPG") {
                    bail!("GPG signatures cannot be created, only verified with pincers")
                }
                let digest = hash_instance(&algo.unwrap(), &checksum)?;
                let computed = compute_hash(digest, &data);
                println!("{} {} {}", &url.unwrap(), &algo.unwrap(), &computed);
                if !args.is_present("quiet") && read::is_remote(&url.unwrap()) {
                    let mut dir = env::temp_dir();
                    dir.push(format!("pincers_{}.tmp", Uuid::new_v4()));
                    let mut f = File::create(&dir)?;
                    f.write_all(&data)?;
                    println!(
                        "# Please check the file contents are as expected, file available at {}",
                        dir.as_path().display().to_string()
                    );
                }
                Ok(None)
            }
            "verify" | "run" => {
                if algo == Some("GPG") {
                    if gpg::verify(&data, &checksum.unwrap()).is_err() {
                        bail!("Signature invalid");
                    }
                    info!("Signature verified");
                } else {
                    let digest = hash_instance(&algo.unwrap(), &checksum)?;
                    let computed = compute_hash(digest, &data);
                    if !hash_matches(&computed, &checksum.unwrap()) {
                        bail!("Checksum invalid");
                    }
                    info!("Checksum verified");
                }
                if command == "run" {
                    let shell = args.value_of("command").unwrap();
                    let extra = args.values_of("extra");
                    let (shell, args) = cli::arg_resplit(shell, extra);
                    Ok(Some(exec(shell, args, &data)?))
                } else {
                    Ok(None)
                }
            }
            _ => bail!("Unknown command"),
        }
    } else {
        bail!("No valid subcommand given");
    }
}

#[test]
fn verify_matches() {
    let hashes = vec![
        vec!["MD5", "be92ab994901c38365cf28a8874fc7c3"],
        vec!["SHA1", "b8aab367f895494d8452a5e89ccfa2b0acb13e90"],
        vec!["SHA2", "ac153c840ff6b48853eb5dca8ff3f5f4f48a7c5e73cc2ef9f50ec672ad670c22612492eae3b7100f51e3f5900ce18cb3ebabe5dbd9fb514d78b3cfa7306165ba"],
        vec!["SHA3", "8844273dccb5f098a14de9cd3cdf250f87693713e6911bcb103545edadae5d7965c14107f238e6e66847f38f471894c007b3cc862f794275809032bfe83d182c"],
    ];
    for hash in hashes {
        let cli = cli::args();
        let mut args = vec!["pincers", "verify", "tests/fixtures/echo.sh"];
        args.extend_from_slice(&hash);
        assert!(run(cli.get_matches_from(args)).unwrap().is_none());
    }
}

#[test]
fn verify_not_matches() {
    let hashes = vec![
        vec!["MD5", "0"],
        vec!["MD5", "00000000000000000000000000000000"],
        vec!["SHA1", "0000000000000000000000000000000000000000"],
        vec!["SHA2", "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"],
        vec!["SHA3", "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"],
    ];
    for hash in hashes {
        let cli = cli::args();
        let mut args = vec!["pincers", "verify", "tests/fixtures/echo.sh"];
        args.extend_from_slice(&hash);
        assert!(run(cli.get_matches_from(args)).is_err());
    }
}

fn main() {
    let args = cli::args().get_matches();
    setup_logging(args.occurrences_of("v")).expect("Could not setup logging");
    let status = run(args);
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

#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate digest;
extern crate hex;
extern crate md5;
extern crate reqwest;
extern crate sha1;
extern crate sha2;
extern crate sha3;

mod cli;
mod hashing;

use clap::ArgMatches;
use digest::DynDigest;
use failure::Error;
use hashing::{digest, HashType};
use hex::encode;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

fn hash_instance(args: &ArgMatches) -> Result<Box<DynDigest>, Error> {
    let hash = if args.is_present("md5") {
        Some(HashType::MD5)
    } else if args.is_present("sha1") {
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
        Err(format_err!("No checksum provided to verify against"))
    }
}

fn download(url: &str) -> Result<Vec<u8>, Error> {
    println!("Getting script: {}", url);
    let mut body = Vec::<u8>::new();
    reqwest::get(url)?
        .error_for_status()?
        .read_to_end(&mut body)?;
    Ok(body)
}

fn check_hash(mut digest: Box<DynDigest>, body: &Vec<u8>, known: &str) -> bool {
    digest.input(body);
    let calculated = encode(digest.result());
    println!("Hash of downloaded script: {}", calculated);
    let equal = calculated.eq_ignore_ascii_case(known);
    match equal {
        true => println!("Hash matches"),
        false => println!("Hash does not match"),
    };
    equal
}

fn run(downloaded: &Vec<u8>) {
    let mut child = Command::new("sh")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn child process");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(downloaded)
        .expect("Failed to write to stdin");
}

fn main() -> Result<(), Error> {
    let args = cli::args();
    let digest = hash_instance(&args)?;
    let checksum = args.value_of("hash").unwrap();
    if let Some(url) = args.value_of("URL") {
        let data = download(&url)?;
        if check_hash(digest, &data, &checksum) {
            run(&data);
        }
    }
    Ok(())
}

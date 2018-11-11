#[macro_use]
extern crate clap;
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
use hashing::{digest, HashType};
use hex::encode;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

fn check_hash(args: &ArgMatches) -> Result<Box<DynDigest>, String> {
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
        Err(format!("No checksum provided to verify against"))
    }
}

fn run(downloaded: &Vec<u8>) {
    let mut child = Command::new("sh")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn child process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(downloaded).expect("Failed to write to stdin");
    }

}

fn main() -> Result<(), String> {
    let args = cli::args();
    let mut digest = check_hash(&args)?;
    if let Some(url) = args.value_of("URL") {
        println!("Getting script: {}", url);
        let mut response = reqwest::get(url).expect("Could not fetch URL");
        if response.status().is_success() {
            let mut body = Vec::<u8>::new();
            response.read_to_end(&mut body).expect("Could not read URL");
            digest.input(&body);
            let calculated = encode(digest.result());
            println!("Hash of downloaded script: {}", calculated);
            match calculated.eq_ignore_ascii_case(args.value_of("hash").unwrap()) {
                true => { println!("Hash matches"); run(&body)},
                false => println!("Hash does not match"),
            }
        }
    }
    Ok(())
}

#[macro_use]
extern crate clap;
extern crate digest;
extern crate md5;
extern crate reqwest;
extern crate sha1;
extern crate sha2;
extern crate sha3;

mod cli;
mod hashing;

use clap::ArgMatches;
use hashing::{digest, HashType};

fn check_hash(args: &ArgMatches) -> () {
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
        let mut boxed = digest(hash.unwrap(), &sum).unwrap();
        // let mut hasher = Box::leak(boxed);
        boxed.input(&[0u8; 12]);
        println!("{:?}", boxed.result());
    }
}

fn main() {
    let args = cli::args();
    check_hash(&args);
    if let Some(url) = args.value_of("URL") {
        println!("Getting script: {}", url);

        let body = reqwest::get(url)
            .expect("Could not fetch URL")
            .text()
            .expect("Could not read body of URL");
        println!("{}", body);
    }
}

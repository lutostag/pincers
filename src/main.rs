#[macro_use]
extern crate clap;
extern crate reqwest;

mod cli;

fn main() {
    let args = cli::args();
    if let Some(url) = args.value_of("URL") {
        println!("Getting script: {}", url);
    }
}

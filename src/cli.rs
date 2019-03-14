use clap::{App, Arg, ArgGroup, ArgMatches};
pub fn args<'a>() -> ArgMatches<'a> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("md5")
                .short("m")
                .long("md5")
                .value_name("MD5")
                .help("MD5 checksum")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("sha1")
                .short("1")
                .long("sha1")
                .value_name("SHA1")
                .help("SHA1 checksum")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("sha2")
                .short("2")
                .long("sha2")
                .value_name("SHA2")
                .help("SHA2 checksum")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("sha3")
                .short("3")
                .long("sha3")
                .value_name("SHA3")
                .help("SHA3 checksum")
                .takes_value(true),
        )
        .group(
            ArgGroup::with_name("hash")
                .args(&["md5", "sha1", "sha2", "sha3"])
                .required(true),
        )
        .arg(
            Arg::with_name("URL")
                .help("Script to run")
                .required(true)
                .index(1),
        )
        .get_matches()
}

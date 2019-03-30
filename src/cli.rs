use clap::{App, AppSettings, Arg, SubCommand};
pub fn args<'a, 'b>() -> App<'a, 'b> {
    let url = Arg::with_name("URL")
        .required(true)
        .help("URL or local file");

    let algo = Arg::with_name("ALGO")
        .possible_values(&["MD5", "SHA1", "SHA2", "SHA3"])
        .required(true)
        .help("Algorithm to use");

    let hash = Arg::with_name("HASH")
        .required(true)
        .help("Expected checksum in hexadecimal");

    App::new(crate_name!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("hash")
                .arg(&url)
                .arg(algo.clone().default_value("SHA3"))
                .about("Generate a checksum for the given input"),
        )
        .subcommand(
            SubCommand::with_name("verify")
                .arg(&url)
                .arg(&algo)
                .arg(&hash)
                .about("Verify the checksum matches the given input"),
        )
        .subcommand(
            SubCommand::with_name("run")
                .arg(&url)
                .arg(&algo)
                .arg(&hash)
                .about("Run the given script if the checksum matches"),
        )
}

use clap::{App, AppSettings, Arg, SubCommand, Values};

pub fn arg_resplit<'a>(command: &'a str, extra: Option<Values<'a>>) -> (&'a str, Vec<&'a str>) {
    let mut split = command.split_whitespace();
    let command = split.next().expect("No command given");
    let mut args = split.collect::<Vec<_>>();
    let mut extra_args = extra.map(|s| s.collect()).unwrap_or_default();
    args.append(&mut extra_args);
    (command, args)
}

pub fn args<'a, 'b>() -> App<'a, 'b> {
    let shell = Arg::with_name("command")
        .help("Command (shell/interpreter) to run the script with")
        .short("c")
        .default_value("sh -s --")
        .hide_default_value(true)
        .takes_value(true)
        .empty_values(false);

    let quiet = Arg::with_name("quiet")
        .help("Do not save remote scripts for review when computing hashes")
        .long("quiet")
        .short("q")
        .takes_value(false);

    let url = Arg::with_name("URL")
        .help("URL or local file")
        .required(true);

    let algo = Arg::with_name("ALGO")
        .help("Algorithm to use")
        .possible_values(&["MD5", "SHA1", "SHA2", "SHA3", "GPG"])
        .required(true);

    let hash = Arg::with_name("HASH")
        .help("Expected checksum in hexadecimal")
        .required(true);

    let extra = Arg::with_name("extra")
        .help("Extra arguments passed to the script")
        .multiple(true)
        .last(true);

    App::new(crate_name!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::DeriveDisplayOrder)
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("v")
                .help("Increases the level of verbosity (the max level is -vvv)")
                .short("v")
                .multiple(true)
                .global(true),
        )
        .subcommand(
            SubCommand::with_name("hash")
                .about("Generate a checksum for the given input")
                .arg(&quiet)
                .arg(&url)
                .arg(algo.clone().default_value("SHA3")),
        )
        .subcommand(
            SubCommand::with_name("verify")
                .about("Verify the checksum/signature matches the given input")
                .arg(&url)
                .arg(&algo)
                .arg(&hash),
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Run the given script if the checksum/signature matches")
                .arg(&shell)
                .arg(&url)
                .arg(&algo)
                .arg(&hash)
                .arg(&extra),
        )
}

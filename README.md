# pincers
A more secure way to run scripts from the web

Because `curl | sh` or `wget | sh` is a little to cowboy for the world we live in.
Use cryptographic hashes to check what is downloaded is what you expect _before_ you run the script.

## Notes
The SHA-1 and MD5 hashes are included so as to work with all widely used hashes. However, these particular hashes have practical demonstrable attacks and should be avoided if at all possible.


## Examples
`$ pincers --sha3 962309203a6dd66167d09558326176175894cdd0809edb950442c910cee868a2 https://sh.rustup.rs
`
`$ pincers --sha2 bb7e8f51023ab8b054c6578591fa0dc361ceeb08744e5cd1f0e551235c4912b2 https://raw.githubusercontent.com/creationix/nvm/v0.33.11/install.sh`

## Usage
```
pincers 0.1.0
Greg Lutostanski <greg.lutostanski@mobilityhouse.com>
A more secure way to run scripts from the web

USAGE:
    pincers [FLAGS] [OPTIONS] <URL> <--md5 <MD5>|--sha1 <SHA1>|--sha2 <SHA2>|--sha3 <SHA3>>

FLAGS:
    -h, --help       Prints help information
    -v               Sets the level of verbosity
    -V, --version    Prints version information

OPTIONS:
    -m, --md5 <MD5>      MD5 checksum
    -1, --sha1 <SHA1>    SHA1 checksum
    -2, --sha2 <SHA2>    SHA2 checksum
    -3, --sha3 <SHA3>    SHA3 checksum

ARGS:
    <URL>    Script to run
```

## Install
It's ~~turtles~~ installers all the way down...

### Cargo
`$ cargo install pincers`

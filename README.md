# pincers
[![Build Status](https://travis-ci.org/lutostag/pincers.svg?branch=master)](https://travis-ci.org/lutostag/pincers) ![Crates.io](https://img.shields.io/crates/v/pincers.svg)

*A more secure way to run scripts from the web*

Because `curl | sh` or `wget | sh` is a little to cowboy for the world we live in.
Use cryptographic hashes to check what is downloaded is what you expect _before_ you run the script.

## Notes
Just because a script from the internet has a checksum, does make it safe or good. It only ensures that all people will get the same version. I would suggest reading and understanding any script from a third party you do not fully trust *before* running it on your machine.

The SHA-1 and MD5 hashes are included so as to work with all widely used hashes. However, these particular hashes have practical demonstrable attacks and should be avoided if at all possible.


## Examples
`$ pincers run https://sh.rustup.rs SHA3 0d420ca95887750de3b95326262cd05911f9c734a82ef66a3280eaa9c738621336321ee0ab0b146279e3dcd01b0134ddac97e2913ce102c344b63bef5cf0b4a9`

`$ pincers run https://raw.githubusercontent.com/creationix/nvm/v0.33.11/install.sh SHA2 bb7e8f51023ab8b054c6578591fa0dc361ceeb08744e5cd1f0e551235c4912b2`

`$ pincers hash https://www.opscode.com/chef/install.sh SHA2`

`$ pincers hash https://nixos.org/nix/install SHA1`

`$ pincers verify https://imgs.xkcd.com/comics/universal_install_script.png SHA3 f855bf08017a89c7d4755df6d909f9f86e4a7cd488470eb27b90f9e22048e451f8d1ec4481fdc44518cfaa709ecde20c67e453d6c2865275fd861664527714d2`

## Usage
```
pincers 0.2.0
Greg Lutostanski <greg.lutostanski@mobilityhouse.com>
A more secure way to run scripts from the web

USAGE:
    pincers [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -v               Sets the level of verbosity
    -V, --version    Prints version information

SUBCOMMANDS:
    hash      Generate a checksum for the given input
    help      Prints this message or the help of the given subcommand(s)
    run       Run the given script if the checksum matches
    verify    Verify the checksum matches the given input
```

## Install
It's ~~turtles~~ installers all the way down...

### Cargo
`$ cargo install pincers`

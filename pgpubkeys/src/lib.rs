#[macro_use]
extern crate failure;

use failure::Error;

mod keybase;

pub fn get(url: &str) -> Result<Vec<u8>, Error> {
    if keybase::matches(url) {
        keybase::download(url)
    } else {
        bail!("No match")
    }
}

#[test]
fn it_works() {
    println!(
        "{:?}",
        std::str::from_utf8(&get("kb:lutostag").unwrap()).unwrap()
    );
    assert_eq!(2 + 2, 4);
}

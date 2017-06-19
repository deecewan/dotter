#[macro_use]
extern crate clap;
extern crate ansi_term;
extern crate toml;
extern crate walkdir;
extern crate ring;
mod checkin;
mod init;
mod print;
mod config;
mod category;
mod secure;
use clap::App;
use checkin::checkin;
use init::init;
use category::category;
use secure::secure;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(init_matches) = matches.subcommand_matches("init") {
        init(init_matches)
    }

    if let Some(checkin_matches) = matches.subcommand_matches("checkin") {
        checkin(checkin_matches)
    }

    if let Some(category_matches) = matches.subcommand_matches("category") {
        category(category_matches)
    }

    if let Some(secure_matches) = matches.subcommand_matches("secure") {
        secure(secure_matches)
    }
}

/*

Done:

- init: creates the dotter directory - not strictly necessary, probably.
- checkin: move a file from wherever it currently is, into dotter, and symlink back
- category: allows for category management (at this stage, only create)

Still to do

- checkout: move the files from their home under ~/dotter to their original location
- secure: mark a file/folder as secure, and encrypt it/it's contents, with the un-encrypted version being symlinked back
- sync: send all dotfiles to github/similar.
- link: from an initial setup, symlink [all|specified category]

I have no idea about the performance impacts, but I imagine the bulk calls to `clone` in checkin are a bad time

*/

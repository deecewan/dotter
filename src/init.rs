extern crate clap;
use clap::ArgMatches;
use std::process::exit;
use std::fs::{metadata, create_dir};
use print::{print_error, print_info, print_success};

fn make_dir(location: &String) {
    let res = create_dir(location);
    match res {
        Ok(_) => {
            print_success(format!("Directory created at: {}", location));
        }
        Err(error) => {
            print_error(format!("Error! {:?}", error));
        }
    }
}

fn check_or_make_dir() {
    let home = env!("HOME");
    print_info(format!("Checking if folder `dotter` exists at {}", home));
    let check_dir = format!("{}/dotter", home);
    let res = metadata(&check_dir);
    match res {
        Ok(_) => {
            let error_string = format!("Directory at `{}` already exists. Exiting...", check_dir);
            print_error(error_string);
            exit(1)
        }
        Err(_) => {
            print_info(format!("Directory {} does not exist. Creating it...", check_dir));
            make_dir(&check_dir)
        }
    };
}

pub fn init(_matches: &ArgMatches) {
    println!("Running the init function!");
    check_or_make_dir();
}

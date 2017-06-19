use config::DEFAULT_FOLDER;
use clap::ArgMatches;
use std::fs::{metadata, create_dir};
use std::string::String;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::io::{ErrorKind};
use print::{print_error, print_success};

fn get_category_path(category: &str) -> PathBuf {
    Path::new(env!("HOME")).join(DEFAULT_FOLDER).join(category)
}

fn category_exists(category_path: &PathBuf) -> bool {
    let meta = metadata(category_path);
    // if meta exists, then the directory must exist
    // otherwise, check the error type and return accordingly
    match meta {
        Ok(meta) => meta.is_dir() || meta.is_file(),
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                return false;
            }
            print_error(format!("{:?}", err.kind()));
            exit(1);
        }
    }
}

// this will be used in other places to create a category
pub fn create_category(matches: &ArgMatches) {
    let category_name = String::from(matches.value_of("name").unwrap());
    let category_path = get_category_path(&category_name);
    if category_exists(&category_path) {
        print_error(format!("Category {} already exists.", category_name));
        exit(1);
    }
    // otherwise, we want to create the folder in the correct place
    let res = create_dir(category_path);
    match res {
        Ok(_) => print_success(format!("Created category '{}'.", &category_name)),
        Err(err) => print_error(format!("Could not create category '{}': {}.", &category_name, err))
    }
}

pub fn category(matches: &ArgMatches) {
    if let Some(create_matches) = matches.subcommand_matches("create") {
        create_category(create_matches);
    }
}

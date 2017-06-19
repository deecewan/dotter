use config::{APPLICATION_NAME, DEFAULT_FOLDER};
use clap::ArgMatches;
use std::fs;
use std::os::unix;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::process::exit;
use print::{print_error, print_success};
use walkdir::WalkDir;

// Allows users to check a new file or directory into dotter
// Usage: dotter checkin [category] [file_or_directory]

/*
The main functionality behind this will be:
- make the category folder in ~/dotter (maybe should confirm, or have a separate command for this?)
- move the existing file into a subdirectory under the dotter category folder
- symlink the file back to it's original location, retaining the same directory structure.

This should set up dotter to have a similar directory structure to GNU Stow, where all the folders
directly under ~/dotter are category names, and then beneath that we have a directory structure that
looks the same as someone's home directory
*/

fn is_a_directory(path: PathBuf) -> Result<bool, Error> {
    let stats = fs::metadata(path);
    return match stats {
               Ok(stats) => Ok(stats.is_dir()),
               Err(err) => Err(err),
           };
}

fn get_new_path(category: &str, path: PathBuf) -> PathBuf {
    let stripped = path.strip_prefix(env!("HOME"))
        .unwrap_or_else(|_| {
                            print_error(format!("That item isn't beneath {}. `{}` only works with items below your home directory.", env!("HOME"), APPLICATION_NAME));
                            exit(1);
                        });
    return Path::new(env!("HOME"))
        .join(DEFAULT_FOLDER) // this should probably be passed in at some point
        .join(category)
        .join(stripped);
}

fn create_directory_at(path: PathBuf) {
    let res = fs::create_dir_all(path);
    match res {
        Ok(_) => {}
        Err(err) => {
            print_error(format!("{}", err));
            exit(1);
        }
    }
}

fn create_directory_structure(category: &str, path: PathBuf) {
    let new_path = get_new_path(category, path.clone());
    let is_dir = is_a_directory(path.clone()).unwrap_or_else(|err| {
                                                                 print_error(format!("{}", err));
                                                                 exit(1);
                                                             });
    if is_dir {
        create_directory_at(new_path);
        return;
    }
    let parent = new_path.parent();
    match parent {
        Some(parent) => {
            create_directory_at(parent.to_path_buf());
        }
        None => {
            print_error(format!("Couldn't find a parent directory for that file!"));
            exit(1);
        }
    }
}

fn normalise_item_location(path_maybe: &str) -> Result<PathBuf, Error> {
    return fs::canonicalize(path_maybe);
}

fn copy_items(category: &str, path: PathBuf) {
    let new_path = get_new_path(category, path.clone());
    let is_dir = is_a_directory(path.clone()).unwrap_or_else(|err| {
                                                                 print_error(format!("{}", err));
                                                                 exit(1);
                                                             });
    if !is_dir {
        // we can just copy the file across without any problems
        let res = fs::copy(path, new_path);
        match res {
            Ok(_) => {}
            Err(err) => {
                print_error(format!("{}", err));
                exit(1);
            }
        }
    } else {
        for entry in WalkDir::new(path) {
            let entry = entry.unwrap();
            // get the manifest
            let metadata = fs::metadata(entry.path());
            let item_path = get_new_path(category, entry.path().to_path_buf());
            if metadata.unwrap().is_file() {
                // get the new path for this item
                let res = fs::copy(entry.path(), item_path);
            } else {
                let res = fs::create_dir(item_path);
            }
        }
    }
}

fn remove_originals(path: PathBuf) {
    let is_dir = is_a_directory(path.clone()).unwrap();
    let res;
    if is_dir {
        res = fs::remove_dir_all(path);
    } else {
        res = fs::remove_file(path);
    }

    match res {
        Ok(_) => {}
        Err(err) => {
            print_error(format!("{}", err));
            exit(1);
        }
    }
}

fn symlink(category: &str, path: PathBuf) {
    let new_path = get_new_path(category, path.clone());
    let res = unix::fs::symlink(new_path, path);
}

pub fn checkin(matches: &ArgMatches) {
    let category = matches.value_of("category").unwrap(); // this is required!
    let item = matches.value_of("item").unwrap();
    let path = normalise_item_location(item).unwrap_or_else(|err| {
                                                                print_error(format!("{}", err));
                                                                exit(1);
                                                            });
    create_directory_structure(category, path.clone());
    copy_items(category, path.clone());
    remove_originals(path.clone());
    symlink(category, path.clone());

    print_success(format!("Successfully checked {} into {}", item, category));
}

mod audd;
mod metadata;
mod options;
mod utils;

use crate::{audd::AudD, metadata::Metadata, options::Options};
use clap::{App, Arg};
use log::{error, trace};
use std::{error::Error, fs, path::Path};

fn process_file(path: &Path, audd: &AudD, options: &Options) -> Result<(), Box<dyn Error>> {
    trace!("Processing file: `{}`", path.to_str().unwrap());
    let response = audd.recognize(path)?;
    let metadata = Metadata::from(&response.data)?;
    metadata.tag_file(path, options)?;
    Ok(())
}

fn recurse_directory(path: &Path, audd: &AudD, options: &Options) -> Result<(), Box<dyn Error>> {
    trace!("Processing directory: `{}`", path.to_str().unwrap());
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            recurse_directory(&path, &audd, &options)?;
        } else if path.is_file() && process_file(&path, &audd, &options).is_err() {
            error!("Unable to process file: `{}`", path.to_str().unwrap());
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let matches = App::new("mtag")
        .version("0.1.0")
        .author("Daniel Wolbach <octowaddle@protonmail.com>")
        .about("Automatically recognize and tag music files.")
        .arg(
            Arg::new("api-token")
                .short('t')
                .long("api-token")
                .value_name("API_TOKEN")
                .about("Sets the AudD API token.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("root")
                .short('r')
                .long("root")
                .value_name("ROOT")
                .about("Choose a root directory.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("enable-directory-change")
                .long("enable-directory-change")
                .about("Changes the directories of the music files to ROOT/ARTIST/ALBUM."),
        )
        .get_matches();

    let audd = AudD::new(matches.value_of("api-token").unwrap());

    let expanded_directory: String =
        shellexpand::tilde(&matches.value_of("root").unwrap()).to_string();
    let root = Path::new(&expanded_directory);

    let options = Options {
        enable_directory_change: matches.is_present("enable-directory-change"),
        root: root.to_path_buf(),
    };

    recurse_directory(&root, &audd, &options)?;

    Ok(())
}

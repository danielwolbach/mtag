use reqwest::blocking::Client;
use std::{
    error::Error,
    fs::{self, File},
    io::{self, Read},
    path::{Path, PathBuf},
};

pub fn file_as_base64(path: &Path) -> Result<String, Box<dyn Error>> {
    let mut f = File::open(&path)?;
    let metadata = fs::metadata(&path)?;
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer)?;
    Ok(base64::encode(buffer))
}

pub fn download_file(url: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let mut response = client.get(url).send()?;
    let mut file = File::create(path)?;
    io::copy(&mut response, &mut file)?;
    Ok(())
}

pub fn change_file_name(path: &Path, name: &str) -> PathBuf {
    let mut result = path.to_owned();
    result.pop();
    result.push(name);
    if let Some(ext) = path.extension() {
        result.set_extension(ext);
    }
    result
}

use std::fs::{self, File};
use std::io;
use std::path::PathBuf;
use tar::Archive;
use failure::Error;
use reqwest;

use xdg_dirs;

const FILE_URL: &'static str =
    "https://dl.dropboxusercontent.com/s/mwgrnsgd8m718sn/Timbre.tar?dl=0";

pub fn sync_files() -> Result<(), Error> {
    fs::remove_dir_all(&*xdg_dirs::SAMPLES)?;

    download_archive()
        .and_then(extract_archive)
        .and_then(|path| {
            fs::rename(path, &*xdg_dirs::SAMPLES)?;
            Ok(())
        })
}

fn download_archive() -> Result<PathBuf, Error> {
    let mut res = reqwest::get(FILE_URL)?;
    let out_path = PathBuf::from(xdg_dirs::DATA.join("Timbre.tar"));
    let mut out_file = File::create(&out_path)?;
    io::copy(&mut res, &mut out_file)?;

    Ok(out_path)
}

fn extract_archive(file_path: PathBuf) -> Result<PathBuf, Error> {
    let file = File::open(&file_path)?;
    let mut a = Archive::new(file);
    a.unpack(&*xdg_dirs::DATA)?;
    Ok(xdg_dirs::DATA.join("Timbre"))
}

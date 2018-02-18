use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use toml;

use xdg_dirs;

lazy_static! {
    static ref FILE_PATH: PathBuf = xdg_dirs::CONFIG.join("config.toml");
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {}

impl Config {
    fn new() -> Config {
        Config {}
    }

    pub fn load() -> Config {
        let mut serialized = String::new();

        fs::File::open(FILE_PATH.clone())
            .map_err(Error::Io)
            .and_then(|mut file| file.read_to_string(&mut serialized).map_err(Error::Io))
            .and_then(|_| toml::from_str(&serialized).map_err(Error::Toml))
            .unwrap_or_default()
    }

    pub fn save(&self) {
        let serialized = toml::to_string(&self).unwrap();

        fs::File::create(FILE_PATH.clone())
            .and_then(|mut file| file.write_all(serialized.as_bytes()))
            .expect("Couldn't write file");
    }
}

impl Default for Config {
    fn default() -> Config {
        Config::new()
    }
}

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Toml(toml::de::Error),
}

use std::{env, fs};
use std::io::{self, Read, Write};
use std::path::PathBuf;

use toml;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub notes_path: String,
    pub tonal_centers_path: String,
}

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Toml(toml::de::Error),
}

impl Config {
    fn new() -> Config {
        let samples_path = Config::app_data_path().join("samples");
        let notes_path = samples_path.join("notes");
        let tonal_centers_path = samples_path.join("tonal-centers");

        Config {
            notes_path: notes_path.to_str().unwrap().to_string(),
            tonal_centers_path: tonal_centers_path.to_str().unwrap().to_string(),
        }
    }

    fn config_path() -> PathBuf {
        Config::app_data_path().join("config.toml")
    }

    fn app_data_path() -> PathBuf {
        let home_dir = env::var("HOME").unwrap();
        let path = PathBuf::from(home_dir).join(".timbre");

        if !path.exists() {
            fs::create_dir(&path);
        }

        path
    }

    pub fn load() -> Config {
        let path = Config::config_path();
        let mut serialized = String::new();

        fs::File::open(path)
            .map_err(Error::Io)
            .and_then(|mut file| file.read_to_string(&mut serialized).map_err(Error::Io))
            .and_then(|_| toml::from_str(&serialized).map_err(Error::Toml))
            .unwrap_or_default()
    }

    pub fn save(&self) {
        let serialized = toml::to_string(&self).unwrap();

        fs::File::create(Config::config_path())
            .and_then(|mut file| file.write_all(serialized.as_bytes()))
            .expect("Couldn't write file");
    }
}

impl Default for Config {
    fn default() -> Config {
        Config::new()
    }
}

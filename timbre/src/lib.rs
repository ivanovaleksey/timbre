extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate ears;
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate tar;
extern crate toml;
extern crate xdg;

pub fn establish_connection() -> diesel::SqliteConnection {
    use diesel::prelude::*;
    let database_url = xdg_dirs::DATA.join("timbre.db");
    diesel::SqliteConnection::establish(database_url.to_str().unwrap())
        .expect(&format!("Error connecting to {}", database_url.display()))
}

pub mod games;
pub mod sampler;
pub mod schema;

pub mod xdg_dirs {
    use std::path::PathBuf;
    use xdg;

    lazy_static!{
        pub static ref BASE: xdg::BaseDirectories =
            xdg::BaseDirectories::with_prefix("timbre").unwrap();

        pub static ref CONFIG: PathBuf =
            BASE.create_config_directory(BASE.get_config_home()).unwrap();

        pub static ref DATA: PathBuf =
            BASE.create_data_directory(BASE.get_data_home()).unwrap();

        pub static ref SAMPLES: PathBuf =
            BASE.create_data_directory("samples").unwrap();
    }
}

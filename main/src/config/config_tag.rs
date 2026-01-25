use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::Path,
};

use bevy::log::info;
use serde::{Deserialize, Serialize};

pub trait ConfigTag {}
pub trait Config {
    fn load_cfg(path: &str) -> Self;
    fn save_cfg(&self, path: &str);
}

impl<T> Config for T
where
    T: ConfigTag + Serialize + for<'de> Deserialize<'de>,
{
    fn load_cfg(path: &str) -> T {
        let file = File::open(path);
        info!("Loading config file at {path}");
        if let Err(err) = file {
            panic!("Error opening file {err:?}");
        }

        let reader = BufReader::new(file.unwrap());

        let json: Result<T, serde_json::Error> = serde_json::from_reader(reader);
        match json {
            Ok(cfg) => return cfg,
            Err(err) => panic!("Error reading JSON from path {path}, {err:?}"),
        }
    }

    fn save_cfg(&self, path_str: &str) {
        info!("Saving config file at {path_str}");
        let path = Path::new(path_str);
        if !path.exists() {
            File::create(path).unwrap();
        }

        let writer = match fs::OpenOptions::new().truncate(true).write(true).open(path) {
            Ok(file) => BufWriter::new(file),
            Err(err) => panic!("Error Opening file at path {path_str}, {err:?}"),
        };

        if let Err(err) = serde_json::to_writer(writer, self) {
            panic!("Error writing JSON to path {path_str}, {err:?}")
        }
    }
}

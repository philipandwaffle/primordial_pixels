use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::{MAIN_SEPARATOR_STR as SEP, Path},
};

use bevy::log::info;
use serde::{Deserialize, Serialize};

pub trait ConfigTag {}
pub trait Config {
    fn load_cfg(path: &Path) -> Self;
    fn save_cfg(&self, path: &Path);
}

impl<T> Config for T
where
    T: ConfigTag + Serialize + for<'de> Deserialize<'de>,
{
    fn load_cfg(path: &Path) -> T {
        let file = File::open(path);
        info!("Loading config file at {path:?}");
        if let Err(err) = file {
            panic!("Error opening file {err:?}");
        }

        let reader = BufReader::new(file.unwrap());

        let json: Result<T, serde_json::Error> = serde_json::from_reader(reader);
        match json {
            Ok(cfg) => return cfg,
            Err(err) => panic!("Error reading JSON from path {path:?}, {err:?}"),
        }
    }

    fn save_cfg(&self, path: &Path) {
        info!("Saving config file at {path:?}");
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    panic!("Error creating parent dir: {parent:?}")
                }
            }
        }

        let writer = match fs::OpenOptions::new().create(true).write(true).open(path) {
            Ok(file) => BufWriter::new(file),
            Err(err) => panic!("Error opening file at path {path:?}, {err:?}"),
        };

        if let Err(err) = serde_json::to_writer(writer, self) {
            panic!("Error writing JSON to path {path:?}, {err:?}")
        }
    }
}

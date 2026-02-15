use std::{
    fs::{create_dir, read_dir},
    path::Path,
};

use bevy::log::error;

use crate::{config::config_tag::Config, world::organism::seed::Seed};

pub struct SeedPacket {
    pub(crate) seeds: Vec<Seed>,
}
impl SeedPacket {
    pub fn new(seeds: Vec<Seed>) -> Self {
        return Self { seeds };
    }

    fn create_dirs(dir: &str) {
        if !Path::new(dir).exists() {
            if let Err(err) = create_dir(dir) {
                error!("Error creating dir {dir}, {err}");
            }
        }

        let seeds_dir = &format!("{dir}/seeds");
        if let Err(err) = create_dir(seeds_dir) {
            error!("Error creating dir {seeds_dir}, {err}");
        }
    }
}
impl Config for SeedPacket {
    fn load_cfg(path: &Path) -> Self {
        let mut me = Self { seeds: vec![] };

        let path_buf = path.to_path_buf();
        let binding = path_buf.join(Path::new("/seeds"));
        let seed_dir = binding.as_path();

        match read_dir(seed_dir) {
            Ok(files) => {
                for file in files {
                    let file_path = file.unwrap().path();
                    let file_path_str = file_path.as_os_str().to_str().unwrap();
                    let s = Seed::load_cfg(&Path::new(&file_path_str.replace("\\", "/")));
                    me.seeds.push(s);
                }
            }
            Err(err) => panic!("Error reading dir {seed_dir:?}, {err:?}"),
        }

        return me;
    }

    fn save_cfg(&self, path: &Path) {
        let path_buf = path.to_path_buf();
        for i in 0..self.seeds.len() {
            self.seeds[i].save_cfg(path_buf.join(Path::new("/seeds/{i}.json")).as_path());
        }
    }
}

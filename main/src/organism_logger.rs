use std::{
    fs::{create_dir, read_dir},
    path::Path,
};

use bevy::{
    app::{Plugin, Update},
    ecs::message::{Message, MessageReader},
    log::error,
};

use crate::{config::config_tag::Config, world::organism::seed::Seed};

pub struct SavePlugin {
    save_path: String,
}
impl SavePlugin {
    pub fn new(save_path: &str) -> Self {
        return Self {
            save_path: save_path.to_string(),
        };
    }
}
impl Plugin for SavePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let log_properties = SaveProperties {
            dir: self.save_path.clone(),
        };

        app.add_event::<LogOrganismsEvent>().add_systems(
            Update,
            move |log_ev: MessageReader<LogOrganismsEvent>| {
                log_organisms(log_ev, &log_properties);
            },
        );
    }
}

struct SaveProperties {
    dir: String,
}

#[derive(Message)]
pub struct LogOrganismsEvent {
    seeds: Vec<Seed>,
    path: String,
}
impl LogOrganismsEvent {
    pub fn new(seeds: Vec<Seed>, path: String) -> Self {
        return Self { seeds, path };
    }
}

fn log_organisms(mut log_ev: MessageReader<LogOrganismsEvent>, log_properties: &SaveProperties) {
    for ev in log_ev.read() {
        SeedPacket::new((ev.seeds).to_vec())
            .save_cfg(&format!("{}/{}", log_properties.dir, ev.path));
    }
}

struct SeedPacket {
    seeds: Vec<Seed>,
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
    fn load_cfg(path: &str) -> Self {
        let mut me = Self { seeds: vec![] };

        let seed_dir = &format!("{path}/seeds");
        match read_dir(seed_dir) {
            Ok(files) => {
                for file in files {
                    let file_path = file.unwrap().path();
                    let file_path_str = file_path.as_os_str().to_str().unwrap();
                    let s = Seed::load_cfg(&file_path_str.replace("\\", "/"));
                    me.seeds.push(s);
                }
            }
            Err(err) => panic!("Error reading dir {seed_dir}, {err:?}"),
        }

        return me;
    }

    fn save_cfg(&self, path: &str) {
        Self::create_dirs(path);
        for i in 0..self.seeds.len() {
            self.seeds[i].save_cfg(&format!("{path}/seeds/{i}.json"));
        }
    }
}

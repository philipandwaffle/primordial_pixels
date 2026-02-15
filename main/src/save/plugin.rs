use std::{
    fs::{self, create_dir, read_dir},
    path::Path,
};

use avian2d::parry::query;
use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        message::{Message, MessageReader, MessageWriter},
        resource::Resource,
        system::{Commands, Query, Res},
    },
    input::{ButtonInput, keyboard::KeyCode},
    log::error,
    transform::components::Transform,
};
use chrono::Local;
use my_derive::ConfigTag;
use serde::{Deserialize, Serialize};

use crate::{
    assets::handles::Handles,
    config::config_tag::{Config, ConfigTag},
    save::{message::LogOrganismsMsg, resource::SaveInfo, seed_packet::SeedPacket},
    world::organism::{component::organism::OrganismMarker, message::SpawnOrganismMsg, seed::Seed},
};

#[derive(ConfigTag, Serialize, Deserialize, Clone, Resource)]
pub struct SavePlugin {
    log_dir: String,
    load_dir: Option<String>,
}
impl Plugin for SavePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_message::<LogOrganismsMsg>()
            .insert_resource(SaveInfo::new(self.log_dir.clone(), self.load_dir.clone()))
            .add_systems(Startup, Self::load_world)
            .add_systems(Update, (Self::log_organisms, Self::save_world));
    }
}
impl SavePlugin {
    fn load_world(
        mut commands: Commands,
        save_info: Res<SaveInfo>,
        handles: Res<Handles>,
        mut spawn_organism_msg: MessageWriter<SpawnOrganismMsg>,
    ) {
        if let Some(load_dir) = &save_info.load_dir {
            let seed_packet = SeedPacket::load_cfg(Path::new(load_dir));

            for seed in seed_packet.seeds {
                spawn_organism_msg.write(Into::<SpawnOrganismMsg>::into(seed));
            }
        }
    }
    fn save_world(
        keys: Res<ButtonInput<KeyCode>>,
        organism_query: Query<(&OrganismMarker, &Transform)>,
        mut save_message: MessageWriter<LogOrganismsMsg>,
        save_info: Res<SaveInfo>,
    ) {
        if !keys.just_released(KeyCode::Enter) {
            return;
        }

        let seeds = organism_query
            .iter()
            .map(|(organism_marker, trans)| organism_marker.as_seed(trans.translation.truncate()))
            .collect::<Vec<Seed>>();

        // let id = fs::read_dir(save_info.log_dir.clone()).iter().len();
        save_message.write(LogOrganismsMsg::new(
            seeds,
            format!("{}", Local::now().format("%d-%m-%Y_%H-%M-%S-%3f")),
        ));
    }

    fn log_organisms(mut log_ev: MessageReader<LogOrganismsMsg>, save_info: Res<SaveInfo>) {
        for ev in log_ev.read() {
            SeedPacket::new((ev.seeds).to_vec())
                .save_cfg(&Path::new(&format!("{}/{}", save_info.log_dir, ev.path)));
        }
    }
}

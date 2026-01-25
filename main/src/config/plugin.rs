use bevy::app::Plugin;

use crate::{
    config::{config::Config as CFG, config_tag::Config},
    consts::CONFIG_PATH,
};

pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let config = CFG::load_cfg(CONFIG_PATH);
        app.insert_resource(config.camera)
            .insert_resource(config.organism)
            .insert_resource(config.node);
    }
}

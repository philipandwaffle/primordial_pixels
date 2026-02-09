use bevy::{app::Plugin, math::Vec2};
use my_derive::ConfigTag;
use serde::{Deserialize, Serialize};

use crate::{config::config_tag::ConfigTag, world::environment::plugin::EnvironmentPlugin};

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy)]
pub struct PetriDishPlugin {
    pub size: Vec2,
    pub display_update_interval: f32,
}
impl Plugin for PetriDishPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(EnvironmentPlugin::new(
            self.size,
            self.display_update_interval,
        ));
    }
}
impl PetriDishPlugin {}

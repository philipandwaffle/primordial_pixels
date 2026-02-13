use avian2d::prelude::Gravity;
use bevy::app::Plugin;

use crate::{
    config::{config::Config as CFG, config_tag::Config},
    consts::CONFIG_PATH,
    performance_info::plugin::PerformanceInfoPlugin,
    save::plugin::SavePlugin,
};

pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let config = load_config();
        app.insert_resource(Gravity(config.physics.gravity_scale))
            .insert_resource(config.camera)
            .insert_resource(config.organism.mutation)
            .insert_resource(config.organism.metabolism)
            .insert_resource(config.organism.transput);

        if config.performance_debug {
            app.add_plugins(PerformanceInfoPlugin);
        }

        app.add_plugins(config.save);
        if let Some(runner) = config.runner {
            app.add_plugins(runner);
        } else if let Some(petri_dish) = config.petri_dish {
            app.add_plugins(petri_dish);
        }
    }
}
pub fn load_config() -> CFG {
    CFG::load_cfg(CONFIG_PATH)
}

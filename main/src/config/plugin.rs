use std::path::Path;

use avian2d::prelude::{Gravity, PhysicsDebugPlugin};
use bevy::app::Plugin;

use crate::{
    config::{config::Config as CFG, config_tag::Config},
    consts::{CONFIG_PATH, KERNEL_CELLS},
    debug::node::NodeDebugPlugin,
    debug::performance::PerformanceDebugPlugin,
    world::organism::distribution::Distribution,
};

pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let config = load_config();
        app.insert_resource(Gravity(config.physics.gravity_scale))
            .insert_resource(config.camera)
            .insert_resource(config.environment)
            .insert_resource(config.organism.mutation)
            .insert_resource(config.organism.metabolism)
            .insert_resource(config.organism.storage)
            .insert_resource(config.organism.transput);

        if config.debug.physics {
            app.add_plugins(PhysicsDebugPlugin);
        }
        if config.debug.performance {
            app.add_plugins(PerformanceDebugPlugin);
        }
        if config.debug.node {
            app.add_plugins(NodeDebugPlugin);
        }

        app.add_plugins(config.save);
        // if let Some(runner) = config.runner {
        //     app.add_plugins(runner);
        // } else
        if let Some(petri_dish) = config.petri_dish {
            app.add_plugins(petri_dish);
        }
    }
}
pub fn load_config() -> CFG<KERNEL_CELLS> {
    let mut cfg = CFG::load_cfg(Path::new(CONFIG_PATH));
    cfg.organism.mutation.type_distribution.normalise();
    cfg.organism.mutation.body_distribution.normalise();
    cfg.organism.mutation.stats_distribution.normalise();

    cfg
}

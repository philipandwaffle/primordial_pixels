use bevy::app::Plugin;

use crate::{
    config::{config::Config as CFG, config_tag::Config},
    consts::CONFIG_PATH,
    organism_logger::SavePlugin,
    performance_info::plugin::PerformanceInfoPlugin,
    util::ticker::Ticker,
    world::environment::plugin::EnvironmentPlugin,
};

pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let config = CFG::load_cfg(CONFIG_PATH);
        app.insert_resource(config.camera)
            .insert_resource(config.organism)
            .insert_resource(config.transput);

        if let Some(runner) = config.runner {
            app.add_plugins((SavePlugin::new("./saves"), runner));
        }

        if config.performance_debug {
            app.add_plugins(PerformanceInfoPlugin);
        }

        app.insert_resource(Ticker::new(config.environment.display_update_interval))
            .add_plugins(EnvironmentPlugin::new(
                config.environment.size,
                config.environment.display_update_interval,
            ));
    }
}

use bevy::{
    app::{Plugin, PreStartup, Update},
    audio::SeekError,
    ecs::system::{Commands, Res, ResMut},
    math::Vec2,
    time::Time,
};

use crate::{
    config::config::Environment as EnvironmentConfig,
    consts::{ENV_CELLS, KERNEL_CELLS},
    world::environment::{
        display::plugin::DisplayPlugin,
        environment::{ConcreteEnv, Environment},
        layer::layer_key::LayerKey,
    },
};

pub struct EnvironmentPlugin {
    display_update_interval: f32,
}
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(DisplayPlugin::new(self.display_update_interval))
            .add_systems(PreStartup, Self::insert_env)
            .add_systems(Update, Self::update_env);
    }
}
impl EnvironmentPlugin {
    pub fn new(display_update_interval: f32) -> Self {
        Self {
            display_update_interval,
        }
    }

    fn insert_env(
        mut commands: Commands,
        environment_config: Res<EnvironmentConfig<KERNEL_CELLS>>,
    ) {
        commands.insert_resource(Environment::<ENV_CELLS, KERNEL_CELLS>::new(
            &environment_config,
        ));
    }

    fn update_env(time: Res<Time>, mut env: ResMut<ConcreteEnv>) {
        env.update_layers(time.delta_secs(), vec![LayerKey::Energy]);
    }
}

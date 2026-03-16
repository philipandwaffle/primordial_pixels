use bevy::{
    app::{Plugin, Update},
    audio::SeekError,
    ecs::system::{Res, ResMut},
    math::Vec2,
    time::Time,
};

use crate::{
    consts::{ENV_CELLS, KERNEL_CELLS},
    world::environment::{
        display::plugin::DisplayPlugin, environment::{ConcreteEnv, Environment}, layer::layer_key::LayerKey,
    },
};

pub struct EnvironmentPlugin {
    size_len: f32,
    display_update_interval: f32,
}
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(Environment::<ENV_CELLS, KERNEL_CELLS>::new(self.size_len))
            .add_plugins(DisplayPlugin::new(self.display_update_interval))
            .add_systems(Update, Self::update_env);
    }
}
impl EnvironmentPlugin {
    pub fn new(size_len: f32, display_update_interval: f32) -> Self {
        Self {
            size_len,
            display_update_interval,
        }
    }

    fn update_env(time: Res<Time>, mut env: ResMut<ConcreteEnv>) {
        env.update_layers(time.delta_secs(), vec![LayerKey::Energy]);
    }
}

use bevy::{
    app::{Plugin, Update},
    audio::SeekError,
    ecs::system::{Res, ResMut},
    math::Vec2,
    time::Time,
};

use crate::{
    consts::{KN, N},
    world::environment::{display::plugin::DisplayPlugin, environment::Environment},
};

pub struct EnvironmentPlugin {
    size: Vec2,
    display_update_interval: f32,
}
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(Environment::<N, KN>::new(self.size))
            .add_plugins((DisplayPlugin::new(self.display_update_interval)))
            .add_systems(Update, Self::update_env);
    }
}
impl EnvironmentPlugin {
    pub fn new(size: Vec2, display_update_interval: f32) -> Self {
        Self {
            size,
            display_update_interval,
        }
    }

    fn update_env(time: Res<Time>, mut env: ResMut<Environment<N, KN>>) {
        env.update(time.delta_secs());
    }
}

use bevy::{
    app::{App, Plugin, PreStartup},
    asset::Assets,
    ecs::system::{Commands, ResMut},
    mesh::Mesh,
    sprite_render::ColorMaterial,
};

use crate::assets::handles::Handles;

pub struct HandlesPlugin;
impl Plugin for HandlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, Self::setup_handles);
    }
}
impl HandlesPlugin {
    fn setup_handles(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut mats: ResMut<Assets<ColorMaterial>>,
    ) {
        let mut handles = Handles::new();
        handles.setup_meshes(&mut meshes);
        handles.setup_mats(&mut mats);

        commands.insert_resource(handles);
    }
}

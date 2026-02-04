use bevy::{
    ecs::bundle::Bundle,
    math::Vec2,
    mesh::Mesh2d,
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use crate::{
    assets::handles::{Handles, MeshKey},
    consts::DISPLAY_Z,
    world::environment::display::{component::DisplayCell, resource::Display},
};

#[derive(Bundle)]
pub struct DisplayCellBundle {
    display_cell: DisplayCell,
    mesh: Mesh2d,
    mat: MeshMaterial2d<ColorMaterial>,
    trans: Transform,
}
impl DisplayCellBundle {
    pub fn new(pos: Vec2, cell_size: Vec2, d: &Display, h: &Handles) -> Self {
        return Self {
            display_cell: DisplayCell,
            mesh: h.get_mesh2d(&MeshKey::Rectangle),
            mat: MeshMaterial2d(d.colors[0].clone()),
            trans: Transform::from_translation(pos.extend(DISPLAY_Z))
                .with_scale(cell_size.extend(1.0)),
        };
    }
}

use bevy::ecs::{component::Component, entity::Entity};

use crate::world::organism::node_type::NodeType;

#[derive(Component)]
pub struct Joint {
    pub nodes: Vec<NodeType>,
    pub thruster: Option<Entity>,
    pub spike: Option<Entity>,
    pub eye: Option<Entity>,
}
impl Joint {
    pub fn new(
        nodes: &Vec<NodeType>,
        thruster: Option<Entity>,
        spike: Option<Entity>,
        eye: Option<Entity>,
    ) -> Self {
        Self {
            nodes: nodes.clone(),
            thruster,
            spike,
            eye,
        }
    }
}

#[derive(Component)]
pub struct Thruster;

#[derive(Component)]
pub struct Spike;

#[derive(Component)]
pub struct Eye;

#[derive(Component)]
pub struct EyeRay;

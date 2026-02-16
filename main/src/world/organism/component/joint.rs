use bevy::ecs::{component::Component, entity::Entity};

use crate::world::organism::node_type::NodeType;

#[derive(Component)]
pub struct Joint {
    pub nodes: Vec<NodeType>,
    pub thruster: Option<Entity>,
}
impl Joint {
    pub fn new(nodes: &Vec<NodeType>, thruster: Option<Entity>) -> Self {
        Self {
            nodes: nodes.clone(),
            thruster,
        }
    }
}

#[derive(Component)]
pub struct Thruster;

use bevy::ecs::component::Component;

use crate::world::organism::node_type::NodeType;

#[derive(Component)]
pub struct Joint {
    pub nodes: Vec<NodeType>,
}
impl Joint {
    pub fn new(nodes: &Vec<NodeType>) -> Self {
        Self {
            nodes: nodes.clone(),
        }
    }
}

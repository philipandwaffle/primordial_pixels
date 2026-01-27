use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

use crate::world::organism::node_type::NodeType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Joint {
    pub pos: Vec2,
    pub nodes: Vec<NodeType>,
}
impl Joint {
    pub fn new(pos: Vec2, nodes: Vec<NodeType>) -> Self {
        Self { pos, nodes }
    }
}

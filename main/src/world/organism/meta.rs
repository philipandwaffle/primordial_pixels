use bevy::math::{Vec2, VectorSpace};
use serde::{Deserialize, Serialize};

use crate::{
    config::config::Metabolism, consts::JOINT_RADIUS, world::organism::organism::Organism,
};

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Meta {
    pub metabolic_cost: f32,
    pub radius: f32,
}
impl Meta {
    pub fn update(mut self, o: &Organism, metabolism: &Metabolism) -> Self {
        self.metabolic_cost = -(o
            .body
            .joints
            .iter()
            .map(|j| j.nodes.len() as f32)
            .sum::<f32>()
            * metabolism.node
            + (o.body.joints.len() as f32 * metabolism.joint)
            + (o.body.bones.len() as f32 * metabolism.bone)
            + (o.body.muscles.len() as f32 * metabolism.muscle));

        self.radius = o
            .body
            .joints
            .iter()
            .map(|j| j.pos.length())
            .collect::<Vec<f32>>()
            .iter()
            .fold(f32::NEG_INFINITY, |a, &b| a.max(b))
            + JOINT_RADIUS;

        self
    }
}

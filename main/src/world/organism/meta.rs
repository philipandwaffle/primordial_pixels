use serde::{Deserialize, Serialize};

use crate::{
    config::config::{Metabolism, Storage},
    consts::JOINT_RADIUS,
    world::organism::organism::Organism,
};

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Meta {
    pub metabolic_cost: f32,
    pub max_energy: f32,
    pub radius: f32,
}
impl Meta {
    pub fn update(mut self, o: &Organism, metabolism: &Metabolism, storage: &Storage) -> Self {
        let num_nodes = o
            .body
            .joints
            .iter()
            .map(|j| j.nodes.len() as f32)
            .sum::<f32>();
        let num_joints = o.body.joints.len() as f32;
        let num_bones = o.body.bones.len() as f32;
        let num_muscles = o.body.muscles.len() as f32;

        self.metabolic_cost = -(num_nodes * metabolism.node
            + (num_joints * metabolism.joint)
            + (num_bones * metabolism.bone)
            + (num_muscles * metabolism.muscle));

        self.max_energy = num_nodes * storage.node
            + (num_joints * storage.joint)
            + (num_bones * storage.bone)
            + (num_muscles * storage.muscle);

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

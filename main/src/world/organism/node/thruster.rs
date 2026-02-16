use std::collections::VecDeque;

use avian2d::prelude::{Forces, RigidBodyForces, forces::ForcesItem};
use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    config::config::Transput as TransputConfig,
    util::function::{clamp_out_01, z_rot_to_dir},
    world::organism::transput::{Transput, remove_output},
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Thruster {
    pub thrust: f32,
    pub z_rot: f32,
}
impl Thruster {
    pub fn new() -> Self {
        Self {
            thrust: 0.0,
            z_rot: 0.0,
        }
    }

    // pub fn get_thrust(&self) -> Vec2 {
    //     z_rot_to_dir(self.z_rot) * self.thrust
    // }
}
impl Transput<f32, ()> for Thruster {
    fn outputs_consumed(&self) -> usize {
        2
    }

    fn inputs_produced(&self) -> usize {
        0
    }

    fn consume_outputs(
        &mut self,
        energy: &mut f32,
        out: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        dt: f32,
    ) {
        let new_thrust = clamp_out_01(remove_output(out)) * transput_config.thruster_strength;
        let new_z_rot = remove_output(out);

        *energy -= (new_thrust - self.thrust).abs() * transput_config.thruster_efficiency * dt;
        *energy -= (new_z_rot - self.z_rot).abs() * transput_config.thruster_efficiency * dt;

        self.thrust = new_thrust;
        self.z_rot = new_z_rot;
    }

    fn produce_inputs(&mut self, _: &mut f32, _: &mut VecDeque<f32>, _: &TransputConfig, _: ()) {}
}

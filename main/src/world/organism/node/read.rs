use crate::{
    config::config::Transput as TransputConfig,
    util::function::{rand_vec2, rot_output, z_rot_to_dir},
    world::{
        environment::{environment::ConcreteEnv, layer::layer_key::LayerKey},
        organism::transput::{Transput, append_input, remove_output},
    },
};
use bevy::math::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Read {
    state: f32,
    pub z_rot: f32,
    pub z_rot_offset: f32,
    pub dist: f32,
    layer_key: LayerKey,
}
impl PartialEq for Read {
    fn eq(&self, other: &Self) -> bool {
        self.z_rot == other.z_rot && self.dist == self.dist
    }
}
impl Read {
    pub fn new(layer_key: LayerKey, z_rot: f32, dist: f32) -> Self {
        Self {
            state: 0.0,
            z_rot: z_rot,
            z_rot_offset: 0.0,
            dist,
            layer_key,
        }
    }
}
impl Transput<(), (&ConcreteEnv, Vec2, f32)> for Read {
    fn consume_outputs(&mut self, _: &mut f32, out: &mut VecDeque<f32>, _: &TransputConfig, _: ()) {
        self.z_rot_offset = rot_output(remove_output(out));
    }

    fn produce_inputs(
        &mut self,
        energy: &mut f32,
        input: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        (env, pos, dt): (&ConcreteEnv, Vec2, f32),
    ) {
        append_input(
            input,
            env.get_value(
                &self.layer_key,
                pos + (z_rot_to_dir(self.z_rot + self.z_rot_offset) * self.dist),
            ),
        );

        *energy -= transput_config.pheromone_read_efficiency * dt;
    }

    fn outputs_consumed(&self) -> usize {
        1
    }

    fn inputs_produced(&self) -> usize {
        1
    }
}

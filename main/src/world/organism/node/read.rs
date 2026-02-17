use std::collections::VecDeque;

use bevy::math::{Vec2, VectorSpace};
use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};

use crate::{
    config::config::Transput as TransputConfig,
    consts::{ENV_CELLS, JOINT_RADIUS, KERNEL_CELLS},
    util::function::rand_vec2,
    world::{
        environment::{environment::Environment, layer::layer_key::LayerKey},
        organism::transput::{Transput, append_input},
    },
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Read {
    state: f32,
    pub read_offset: Vec2,
    layer_key: LayerKey,
}
impl PartialEq for Read {
    fn eq(&self, other: &Self) -> bool {
        self.read_offset == other.read_offset
    }
}
impl Read {
    pub fn new(layer_key: LayerKey, rng: &mut ThreadRng) -> Self {
        Self {
            state: 0.0,
            read_offset: rand_vec2(rng, JOINT_RADIUS),
            layer_key,
        }
    }
}
impl Transput<(), (&Environment<ENV_CELLS, KERNEL_CELLS>, Vec2, f32)> for Read {
    fn consume_outputs(&mut self, _: &mut f32, _: &mut VecDeque<f32>, _: &TransputConfig, _: ()) {}

    fn produce_inputs(
        &mut self,
        energy: &mut f32,
        input: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        (env, pos, dt): (&Environment<ENV_CELLS, KERNEL_CELLS>, Vec2, f32),
    ) {
        append_input(
            input,
            env.get_value(&self.layer_key, pos + self.read_offset),
        );

        *energy -= transput_config.pheromone_read_efficiency * dt;
    }

    fn outputs_consumed(&self) -> usize {
        0
    }

    fn inputs_produced(&self) -> usize {
        1
    }
}

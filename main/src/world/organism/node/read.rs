use std::collections::VecDeque;

use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    config::config::Transput as TransputConfig,
    consts::{ENV_CELLS, KERNEL_CELLS},
    world::{
        environment::{environment::Environment, layer::layer_key::LayerKey},
        organism::transput::{Transput, append_input},
    },
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Read {
    state: f32,
    layer_key: LayerKey,
}
impl Read {
    pub fn new(layer_key: LayerKey) -> Self {
        Self {
            state: 0.0,
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
        append_input(input, env.get_value(&self.layer_key, pos));

        *energy -= transput_config.pheromone_read_efficiency * dt;
    }

    fn outputs_consumed(&self) -> usize {
        0
    }

    fn inputs_produced(&self) -> usize {
        1
    }
}

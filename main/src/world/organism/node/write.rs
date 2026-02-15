use std::collections::VecDeque;

use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    config::config::Transput as TransputConfig,
    consts::{ENV_CELLS, KERNEL_CELLS},
    util::function::clamp_out_01,
    world::{
        environment::{environment::Environment, layer::layer_key::LayerKey},
        organism::transput::{Transput, remove_output},
    },
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Write {
    state: f32,
    layer_key: LayerKey,
}
impl Write {
    pub fn new(layer_key: LayerKey) -> Self {
        Self {
            state: 0.0,
            layer_key,
        }
    }
}
impl Transput<(&mut Environment<ENV_CELLS, KERNEL_CELLS>, Vec2, f32), ()> for Write {
    fn consume_outputs(
        &mut self,
        energy: &mut f32,
        output: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        (env, pos, dt): (&mut Environment<ENV_CELLS, KERNEL_CELLS>, Vec2, f32),
    ) {
        let max_write =
            clamp_out_01(remove_output(output)) * transput_config.pheromone_write_efficiency * dt;
        let mut delta = max_write;
        env.delta_value(&self.layer_key, pos, &mut delta);
        *energy -= transput_config.pheromone_write_efficiency * dt * (max_write - delta);
    }

    fn produce_inputs(&mut self, _: &mut f32, _: &mut VecDeque<f32>, _: &TransputConfig, _: ()) {}

    fn outputs_consumed(&self) -> usize {
        1
    }

    fn inputs_produced(&self) -> usize {
        0
    }
}

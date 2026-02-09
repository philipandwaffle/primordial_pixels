use std::collections::VecDeque;

use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    config::config::Transput as TransputConfig,
    consts::{KN, N},
    util::function::clamp_out,
    world::{
        environment::{environment::Environment, layer::layer_key::LayerKey},
        organism::{
            node::node::Node,
            transput::{Transput, remove_output},
        },
    },
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PheromoneWrite {
    state: f32,
    layer_id: usize,
}
impl PheromoneWrite {
    pub fn new(layer_id: usize) -> Self {
        Self {
            state: 0.0,
            layer_id,
        }
    }
}
impl Transput<(&mut Environment<N, KN>, Vec2), ()> for PheromoneWrite {
    fn consume_outputs(
        &mut self,
        energy: &mut f32,
        output: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        (env, pos): (&mut Environment<N, KN>, Vec2),
    ) {
        let mut delta =
            clamp_out(remove_output(output)) * transput_config.pheromone_write_efficiency;
        env.delta_value(&LayerKey::Pheromone(self.layer_id), pos, &mut delta);
        *energy -= transput_config.pheromone_write_efficiency - delta;
    }

    fn produce_inputs(&mut self, _: &mut f32, _: &mut VecDeque<f32>, _: &TransputConfig, _: ()) {}

    fn outputs_consumed(&self) -> usize {
        1
    }

    fn inputs_produced(&self) -> usize {
        0
    }
}

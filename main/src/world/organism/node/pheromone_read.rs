use std::collections::VecDeque;

use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    config::config::Transput as TransputConfig,
    consts::{KN, N},
    world::{
        environment::{environment::Environment, layer::layer_key::LayerKey},
        organism::transput::{Transput, append_input},
    },
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PheromoneRead {
    state: f32,
    layer_id: usize,
}
impl PheromoneRead {
    pub fn new(layer_id: usize) -> Self {
        Self {
            state: 0.0,
            layer_id,
        }
    }
}
impl Transput<(), (&Environment<N, KN>, Vec2)> for PheromoneRead {
    fn consume_outputs(&mut self, _: &mut f32, _: &mut VecDeque<f32>, _: &TransputConfig, _: ()) {}

    fn produce_inputs(
        &mut self,
        energy: &mut f32,
        input: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        (env, pos): (&Environment<N, KN>, Vec2),
    ) {
        append_input(
            input,
            env.get_value(&LayerKey::Pheromone(self.layer_id), pos),
        );

        *energy -= transput_config.pheromone_read_efficiency;
    }

    fn outputs_consumed(&self) -> usize {
        0
    }

    fn inputs_produced(&self) -> usize {
        1
    }
}

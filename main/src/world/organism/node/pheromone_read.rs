use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    config::config::Node as NodeConfig,
    consts::{KN, N},
    world::{
        environment::{environment::Environment, layer::layer_key::LayerKey},
        organism::node::node::Node,
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
impl Node<(), (&Environment<N, KN>, Vec2)> for PheromoneRead {
    fn consume_outputs(&mut self, _: &mut f32, _: &mut Vec<f32>, _: &NodeConfig, _: ()) {}

    fn produce_inputs(
        &mut self,
        energy: &mut f32,
        input: &mut Vec<f32>,
        node_config: &NodeConfig,
        (env, pos): (&Environment<N, KN>, Vec2),
    ) {
        input.push(env.get_value(&LayerKey::Pheromone(self.layer_id), pos));

        *energy -= node_config.pheromone_read_efficiency;
    }

    fn outputs_consumed(&self) -> usize {
        0
    }

    fn inputs_produced(&self) -> usize {
        1
    }
}

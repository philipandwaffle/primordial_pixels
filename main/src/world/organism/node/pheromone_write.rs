use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    config::config::Node as NodeConfig,
    consts::{KN, N},
    util::function::clamp_out,
    world::{
        environment::{environment::Environment, layer::layer_key::LayerKey},
        organism::node::node::Node,
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
impl Node<(&mut Environment<N, KN>, Vec2), ()> for PheromoneWrite {
    fn consume_outputs(
        &mut self,
        energy: &mut f32,
        out: &mut Vec<f32>,
        node_config: &NodeConfig,
        (env, pos): (&mut Environment<N, KN>, Vec2),
    ) {
        let mut delta = clamp_out(out.pop().unwrap()) * node_config.pheromone_write_efficiency;
        env.delta_value(&LayerKey::Pheromone(self.layer_id), pos, &mut delta);
        *energy -= node_config.pheromone_write_efficiency - delta;
    }

    fn produce_inputs(&mut self, _: &mut f32, _: &mut Vec<f32>, _: &NodeConfig, _: ()) {}

    fn outputs_consumed(&self) -> usize {
        1
    }

    fn inputs_produced(&self) -> usize {
        0
    }
}

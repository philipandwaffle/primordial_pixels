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
pub struct Energy {
    collected_energy: f32,
}
impl Energy {
    pub fn new() -> Self {
        Self {
            collected_energy: 0.0,
        }
    }
}
impl Node<(&mut Environment<N, KN>, Vec2), ()> for Energy {
    fn consume_outputs(
        &mut self,
        energy: &mut f32,
        _: &mut Vec<f32>,
        node_config: &NodeConfig,
        (env, pos): (&mut Environment<N, KN>, Vec2),
    ) {
        let mut max_collect = node_config.energy_collect_rate;

        env.delta_value(&LayerKey::Energy, pos, &mut max_collect);
        *energy += node_config.energy_collect_rate - max_collect;
    }

    fn produce_inputs(&mut self, _: &mut f32, _: &mut Vec<f32>, _: &NodeConfig, _: ()) {}

    fn outputs_consumed(&self) -> usize {
        0
    }

    fn inputs_produced(&self) -> usize {
        0
    }
}

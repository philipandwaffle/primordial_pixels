use std::collections::VecDeque;

use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    config::config::Transput as TransputConfig,
    consts::{KN, N},
    world::{
        environment::{environment::Environment, layer::layer_key::LayerKey},
        organism::{node::node::Node, transput::Transput},
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
impl Transput<(&mut Environment<N, KN>, Vec2), ()> for Energy {
    fn consume_outputs(
        &mut self,
        energy: &mut f32,
        _: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        (env, pos): (&mut Environment<N, KN>, Vec2),
    ) {
        let mut max_collect = transput_config.energy_collect_rate;

        env.delta_value(&LayerKey::Energy, pos, &mut max_collect);
        *energy += transput_config.energy_collect_rate - max_collect;
    }

    fn produce_inputs(&mut self, _: &mut f32, _: &mut VecDeque<f32>, _: &TransputConfig, _: ()) {}

    fn outputs_consumed(&self) -> usize {
        0
    }

    fn inputs_produced(&self) -> usize {
        0
    }
}

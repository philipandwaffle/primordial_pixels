use std::collections::VecDeque;

use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    config::config::Transput as TransputConfig,
    consts::{ENV_CELLS, KERNEL_CELLS},
    world::{
        environment::{environment::Environment, layer::layer_key::LayerKey},
        organism::transput::Transput,
    },
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Decomposer {
    // collected_energy: f32,
}
impl Decomposer {
    pub fn new() -> Self {
        Self {
            // collected_energy: 0.0,
        }
    }
}
impl Transput<(&mut Environment<ENV_CELLS, KERNEL_CELLS>, Vec2, f32), ()> for Decomposer {
    fn consume_outputs(
        &mut self,
        energy: &mut f32,
        _: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        (env, pos, dt): (&mut Environment<ENV_CELLS, KERNEL_CELLS>, Vec2, f32),
    ) {
        let max_collect = transput_config.decomposer_collect_rate * dt;
        let mut delta = max_collect;

        env.delta_value(&LayerKey::Decompose, pos, &mut delta);
        let collected_energy = max_collect - delta;
        // info!("collected {}", collected_energy);
        *energy += collected_energy;
    }

    fn produce_inputs(&mut self, _: &mut f32, _: &mut VecDeque<f32>, _: &TransputConfig, _: ()) {}

    fn outputs_consumed(&self) -> usize {
        0
    }

    fn inputs_produced(&self) -> usize {
        0
    }
}

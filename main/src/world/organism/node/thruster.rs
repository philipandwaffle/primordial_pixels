use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::{
    config::config::Transput as TransputConfig,
    util::function::clamp_out,
    world::organism::transput::{Transput, remove_output},
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Thruster {
    state: f32,
    z_rot: f32,
}
impl Thruster {
    pub fn new(z_rot: f32) -> Self {
        Self { state: 0.0, z_rot }
    }
}
impl Transput<(), ()> for Thruster {
    fn outputs_consumed(&self) -> usize {
        1
    }

    fn inputs_produced(&self) -> usize {
        0
    }

    fn consume_outputs(
        &mut self,
        energy: &mut f32,
        out: &mut VecDeque<f32>,
        _: &TransputConfig,
        _: (),
    ) {
        let state = self.state;
        let new_state = clamp_out(remove_output(out));

        *energy -= (state - self.state).abs();
        self.state = new_state;
    }

    fn produce_inputs(&mut self, _: &mut f32, _: &mut VecDeque<f32>, _: &TransputConfig, _: ()) {}
}

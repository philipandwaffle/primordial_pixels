use serde::{Deserialize, Serialize};

use crate::{
    config::config::Node as NodeConfig, util::function::clamp_out,
    world::organism::node::node::Node,
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
impl Node<(), ()> for Thruster {
    fn outputs_consumed(&self) -> usize {
        1
    }

    fn inputs_produced(&self) -> usize {
        0
    }

    fn consume_outputs(&mut self, energy: &mut f32, out: &mut Vec<f32>, _: &NodeConfig, _: ()) {
        let state = self.state;
        let new_state = clamp_out(out.pop().unwrap());

        *energy -= (state - self.state).abs();
        self.state = new_state;
    }

    fn produce_inputs(&mut self, _: &mut f32, _: &mut Vec<f32>, _: &NodeConfig, _: ()) {}
}

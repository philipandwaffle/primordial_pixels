use crate::{
    config::config::Transput as TransputConfig,
    world::organism::transput::{Transput, append_input, remove_output},
};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Spike {
    pub state: bool,
}
impl Spike {
    pub fn new() -> Self {
        Self { state: false }
    }
}
impl Transput<f32, ()> for Spike {
    fn outputs_consumed(&self) -> usize {
        1
    }

    fn inputs_produced(&self) -> usize {
        1
    }

    fn consume_outputs(
        &mut self,
        energy: &mut f32,
        out: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        dt: f32,
    ) {
        self.state = remove_output(out) > 0.0;

        if self.state {
            *energy -= transput_config.spike_efficiency * dt;
        }
    }

    fn produce_inputs(
        &mut self,
        _: &mut f32,
        input: &mut VecDeque<f32>,
        _: &TransputConfig,
        _: (),
    ) {
        append_input(input, if self.state { 1.0 } else { -1.0 });
    }
}

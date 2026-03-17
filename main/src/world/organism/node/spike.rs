use crate::{config::config::Transput as TransputConfig, world::organism::transput::Transput};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Spike {
    // pub state: bool,
}
impl Spike {
    pub fn new() -> Self {
        Self {}
    }
}
impl Transput<f32, ()> for Spike {
    fn outputs_consumed(&self) -> usize {
        0
    }

    fn inputs_produced(&self) -> usize {
        0
    }

    fn produce_inputs(&mut self, _: &mut f32, _: &mut VecDeque<f32>, _: &TransputConfig, _: ()) {}

    fn consume_outputs(&mut self, _: &mut f32, _: &mut VecDeque<f32>, _: &TransputConfig, _: f32) {}
}

use std::collections::VecDeque;

use crate::{
    config::config::Transput as TransputConfig,
    world::organism::out_in::OutputConsumedInputProduced,
};
// Output is popped from back
// input is pushed on front

pub trait Transput<C, P> {
    // Return the  and return the energy cost of doing so
    fn produce_inputs(
        &mut self,
        energy: &mut f32,
        input: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        args: P,
    );

    // Update the state and return the energy cost of doing so
    fn consume_outputs(
        &mut self,
        energy: &mut f32,
        output: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        args: C,
    );

    // The number of brain outputs consumed by this node
    fn outputs_consumed(&self) -> usize;
    // The number of brain inputs produced by this node
    fn inputs_produced(&self) -> usize;

    fn out_in(&self) -> OutputConsumedInputProduced {
        return OutputConsumedInputProduced([self.outputs_consumed(), self.inputs_produced()]);
    }
}

// pub trait TransputUtil<C, P> {
//     fn remove_output(output: &mut VecDeque<f32>) -> f32 {
//         output.pop_back().unwrap()
//     }

//     fn append_input(input: &mut VecDeque<f32>, val: f32) {
//         input.push_front(val);
//     }
// }
// impl<C, P, T> TransputUtil<C, P> for T where T: Transput<C, P> {}

pub fn remove_output(output: &mut VecDeque<f32>) -> f32 {
    output.pop_back().unwrap()
}

pub fn append_input(input: &mut VecDeque<f32>, val: f32) {
    input.push_front(val);
}

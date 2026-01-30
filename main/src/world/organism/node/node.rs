use crate::{
    config::config::Node as NodeConfig, world::organism::in_out::OutputConsumedInputProduced,
};

pub trait Node<C, P> {
    // Update the state and return the energy cost of doing so
    // fn update_state(&mut self, node_config: NodeConfig, args: A) -> f32;
    fn consume_outputs(
        &mut self,
        energy: &mut f32,
        out: &mut Vec<f32>,
        node_config: &NodeConfig,
        args: C,
    );
    fn produce_inputs(
        &mut self,
        energy: &mut f32,
        input: &mut Vec<f32>,
        node_config: &NodeConfig,
        args: P,
    );

    // The number of brain outputs consumed by this node
    fn outputs_consumed(&self) -> usize;
    // The number of brain inputs produced by this node
    fn inputs_produced(&self) -> usize;

    fn out_in(&self) -> OutputConsumedInputProduced {
        return OutputConsumedInputProduced([self.outputs_consumed(), self.inputs_produced()]);
    }
}

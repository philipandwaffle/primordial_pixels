use std::{collections::VecDeque, f32::consts::PI};

use bevy::math::Vec2;
use rand::{Rng, rngs::ThreadRng};
use serde::{Deserialize, Serialize};

use crate::{
    config::config::{Mutation as MutationConfig, Transput as TransputConfig},
    consts::{ENV_CELLS, KERNEL_CELLS, PHEROMONE_LAYERS},
    world::{
        environment::{
            environment::{ConcreteEnv, Environment},
            layer::layer_key::LayerKey,
        },
        organism::{
            mutation::mutation::Mut,
            node::{
                decomposer::Decomposer, energy::Energy, read::Read, spike::Spike,
                thruster::Thruster, write::Write,
            },
            organism::Organism,
            transput::Transput,
        },
    },
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum NodeType {
    Energy(Energy),
    Decomposer(Decomposer),
    Read(Read),
    Write(Write),
    Thruster(Thruster),
    Spike(Spike),
}
impl NodeType {
    pub fn can_alter(&self) -> bool {
        match self {
            NodeType::Read(_) => true,
            _ => false,
        }
    }
}
impl PartialEq for NodeType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Energy(_), Self::Energy(_)) => true,
            (Self::Decomposer(_), Self::Decomposer(_)) => true,
            (Self::Read(a), Self::Read(b)) => a == b,
            (Self::Write(_), Self::Write(_)) => true,
            (Self::Thruster(_), Self::Thruster(_)) => true,
            (Self::Spike(_), Self::Spike(_)) => true,
            _ => false,
        }
    }
}
impl Mut for NodeType {
    fn rand(rng: &mut ThreadRng, _: &MutationConfig, _: &Organism) -> Option<Self> {
        Some(match rng.random_range(0..=5) {
            0 => Self::Energy(Energy::new()),
            1 => Self::Decomposer(Decomposer::new()),
            2 => Self::Read(Read::new(LayerKey::rand_read_layer(rng), rng)),
            3 => Self::Write(Write::new(LayerKey::rand_write_layer(rng))),
            4 => Self::Thruster(Thruster::new()),
            _ => Self::Spike(Spike::new()),
        })
    }
}
impl Transput<(&mut ConcreteEnv, Vec2, f32), (&ConcreteEnv, Vec2, f32)> for NodeType {
    fn consume_outputs(
        &mut self,
        e: &mut f32,
        out: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        args: (&mut ConcreteEnv, Vec2, f32),
    ) {
        match self {
            NodeType::Energy(energy) => energy.consume_outputs(e, out, transput_config, args),
            NodeType::Decomposer(decomposer) => {
                decomposer.consume_outputs(e, out, transput_config, args)
            }
            NodeType::Read(pheromone_read) => {
                pheromone_read.consume_outputs(e, out, transput_config, ())
            }
            NodeType::Write(pheromone_write) => {
                pheromone_write.consume_outputs(e, out, transput_config, args)
            }
            NodeType::Thruster(thruster) => {
                thruster.consume_outputs(e, out, transput_config, args.2)
            }
            NodeType::Spike(spike) => {
                spike.consume_outputs(e, out, transput_config, args.2);
            }
        };
    }

    fn produce_inputs(
        &mut self,
        e: &mut f32,
        input: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        args: (&ConcreteEnv, Vec2, f32),
    ) {
        match self {
            NodeType::Energy(energy) => energy.produce_inputs(e, input, transput_config, ()),
            NodeType::Decomposer(decomposer) => {
                decomposer.produce_inputs(e, input, transput_config, ())
            }
            NodeType::Read(pheromone_read) => {
                pheromone_read.produce_inputs(e, input, transput_config, args)
            }
            NodeType::Write(pheromone_write) => {
                pheromone_write.produce_inputs(e, input, transput_config, ())
            }
            NodeType::Thruster(thruster) => thruster.produce_inputs(e, input, transput_config, ()),
            NodeType::Spike(spike) => spike.produce_inputs(e, input, transput_config, ()),
        };
    }

    fn outputs_consumed(&self) -> usize {
        match self {
            NodeType::Energy(energy) => energy.outputs_consumed(),
            NodeType::Decomposer(decomposer) => decomposer.outputs_consumed(),
            NodeType::Read(pheromone_read) => pheromone_read.outputs_consumed(),
            NodeType::Write(pheromone_write) => pheromone_write.outputs_consumed(),
            NodeType::Thruster(thruster) => thruster.outputs_consumed(),
            NodeType::Spike(spike) => spike.outputs_consumed(),
        }
    }

    fn inputs_produced(&self) -> usize {
        match self {
            NodeType::Energy(energy) => energy.inputs_produced(),
            NodeType::Decomposer(decomposer) => decomposer.inputs_produced(),
            NodeType::Read(pheromone_read) => pheromone_read.inputs_produced(),
            NodeType::Write(pheromone_write) => pheromone_write.inputs_produced(),
            NodeType::Thruster(thruster) => thruster.inputs_produced(),
            NodeType::Spike(spike) => spike.inputs_produced(),
        }
    }
}

#[cfg(test)]
mod test {
    use bevy::math::vec2;

    use crate::world::{
        environment::layer::layer_key::LayerKey,
        organism::{node::read::Read, node_type::NodeType},
    };

    #[test]
    fn foo() {
        let mut rng = rand::rng();

        let mut a = Read::new(LayerKey::Energy, &mut rng);
        a.read_offset = vec2(0.0, 0.0);
        let a = NodeType::Read(a);

        let mut b = Read::new(LayerKey::Energy, &mut rng);
        b.read_offset = vec2(0.0, 0.0);
        let b = NodeType::Read(b);

        let nodes = vec![a];
        println!("{:?} {:?} {:?}", a, b, nodes.contains(&b));
    }
}

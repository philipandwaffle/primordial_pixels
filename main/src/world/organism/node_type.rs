use std::{collections::VecDeque, f32::consts::PI};

use bevy::math::Vec2;
use rand::{Rng, rngs::ThreadRng};
use serde::{Deserialize, Serialize};

use crate::{
    config::config::{Mutation as MutationConfig, Transput as TransputConfig},
    consts::{ENV_CELLS, KERNEL_CELLS, PHEROMONE_LAYERS},
    world::{
        environment::{environment::Environment, layer::layer_key::LayerKey},
        organism::{
            mutation::mutation::Mut,
            node::{energy::Energy, read::Read, thruster::Thruster, write::Write},
            organism::Organism,
            transput::Transput,
        },
    },
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum NodeType {
    Energy(Energy),
    Read(Read),
    Write(Write),
    Thruster(Thruster),
}
impl Mut for NodeType {
    fn rand(rng: &mut ThreadRng, _: &MutationConfig, _: &Organism) -> Option<Self> {
        Some(match rng.random_range(0..=3) {
            0 => Self::Energy(Energy::new()),
            1 => Self::Read(Read::new(LayerKey::rand_read_layer(rng))),
            2 => Self::Write(Write::new(LayerKey::rand_write_layer(rng))),
            _ => Self::Thruster(Thruster::new(rng.random_range(-PI..PI))),
        })
    }
}
impl
    Transput<
        (&mut Environment<ENV_CELLS, KERNEL_CELLS>, Vec2, f32),
        (&Environment<ENV_CELLS, KERNEL_CELLS>, Vec2, f32),
    > for NodeType
{
    fn consume_outputs(
        &mut self,
        e: &mut f32,
        out: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        args: (&mut Environment<ENV_CELLS, KERNEL_CELLS>, Vec2, f32),
    ) {
        match self {
            NodeType::Energy(energy) => energy.consume_outputs(e, out, transput_config, args),
            NodeType::Read(pheromone_read) => {
                pheromone_read.consume_outputs(e, out, transput_config, ())
            }
            NodeType::Write(pheromone_write) => {
                pheromone_write.consume_outputs(e, out, transput_config, args)
            }
            NodeType::Thruster(thruster) => {
                thruster.consume_outputs(e, out, transput_config, args.2)
            }
        };
    }

    fn produce_inputs(
        &mut self,
        e: &mut f32,
        input: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        args: (&Environment<ENV_CELLS, KERNEL_CELLS>, Vec2, f32),
    ) {
        match self {
            NodeType::Energy(energy) => energy.produce_inputs(e, input, transput_config, ()),
            NodeType::Read(pheromone_read) => {
                pheromone_read.produce_inputs(e, input, transput_config, args)
            }
            NodeType::Write(pheromone_write) => {
                pheromone_write.produce_inputs(e, input, transput_config, ())
            }
            NodeType::Thruster(thruster) => thruster.produce_inputs(e, input, transput_config, ()),
        };
    }

    fn outputs_consumed(&self) -> usize {
        match self {
            NodeType::Energy(energy) => energy.outputs_consumed(),
            NodeType::Read(pheromone_read) => pheromone_read.outputs_consumed(),
            NodeType::Write(pheromone_write) => pheromone_write.outputs_consumed(),
            NodeType::Thruster(thruster) => thruster.outputs_consumed(),
        }
    }

    fn inputs_produced(&self) -> usize {
        match self {
            NodeType::Energy(energy) => energy.inputs_produced(),
            NodeType::Read(pheromone_read) => pheromone_read.inputs_produced(),
            NodeType::Write(pheromone_write) => pheromone_write.inputs_produced(),
            NodeType::Thruster(thruster) => thruster.inputs_produced(),
        }
    }
}

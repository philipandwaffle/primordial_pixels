use std::f32::consts::PI;

use bevy::math::Vec2;
use rand::{Rng, rngs::ThreadRng};
use serde::{Deserialize, Serialize};

use crate::{
    config::config::{Node as NodeConfig, Organism as OrganismConfig},
    consts::{KN, N, PHEROMONE_LAYERS},
    world::{
        environment::environment::Environment,
        organism::{
            mutation::mutation::Mut,
            node::{
                energy::Energy, node::Node, pheromone_read::PheromoneRead,
                pheromone_write::PheromoneWrite, thruster::Thruster,
            },
            organism::Organism,
        },
    },
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum NodeType {
    Energy(Energy),
    PheromoneRead(PheromoneRead),
    PheromoneWrite(PheromoneWrite),
    Thruster(Thruster),
}
impl Mut for NodeType {
    fn rand(rng: &mut ThreadRng, _: &OrganismConfig, _: &Organism) -> Option<Self> {
        Some(match rng.random_range(0..=2) {
            0 => Self::Energy(Energy::new()),
            1 => Self::PheromoneRead(PheromoneRead::new(rng.random_range(0..PHEROMONE_LAYERS))),
            2 => Self::PheromoneWrite(PheromoneWrite::new(rng.random_range(0..PHEROMONE_LAYERS))),
            _ => Self::Thruster(Thruster::new(rng.random_range(-PI..PI))),
        })
    }
}
impl Node<(&mut Environment<N, KN>, Vec2), (&Environment<N, KN>, Vec2)> for NodeType {
    fn consume_outputs(
        &mut self,
        e: &mut f32,
        out: &mut Vec<f32>,
        node_config: &NodeConfig,
        args: (&mut Environment<N, KN>, Vec2),
    ) {
        match self {
            NodeType::Energy(energy) => energy.consume_outputs(e, out, node_config, args),
            NodeType::PheromoneRead(pheromone_read) => {
                pheromone_read.consume_outputs(e, out, node_config, ())
            }
            NodeType::PheromoneWrite(pheromone_write) => {
                pheromone_write.consume_outputs(e, out, node_config, args)
            }
            NodeType::Thruster(thruster) => thruster.consume_outputs(e, out, node_config, ()),
        };
    }

    fn produce_inputs(
        &mut self,
        e: &mut f32,
        input: &mut Vec<f32>,
        node_config: &NodeConfig,
        args: (&Environment<N, KN>, Vec2),
    ) {
        match self {
            NodeType::Energy(energy) => energy.produce_inputs(e, input, node_config, ()),
            NodeType::PheromoneRead(pheromone_read) => {
                pheromone_read.produce_inputs(e, input, node_config, args)
            }
            NodeType::PheromoneWrite(pheromone_write) => {
                pheromone_write.produce_inputs(e, input, node_config, ())
            }
            NodeType::Thruster(thruster) => thruster.produce_inputs(e, input, node_config, ()),
        };
    }

    fn outputs_consumed(&self) -> usize {
        match self {
            NodeType::Energy(energy) => energy.outputs_consumed(),
            NodeType::PheromoneRead(pheromone_read) => pheromone_read.outputs_consumed(),
            NodeType::PheromoneWrite(pheromone_write) => pheromone_write.outputs_consumed(),
            NodeType::Thruster(thruster) => thruster.outputs_consumed(),
        }
    }

    fn inputs_produced(&self) -> usize {
        match self {
            NodeType::Energy(energy) => energy.inputs_produced(),
            NodeType::PheromoneRead(pheromone_read) => pheromone_read.inputs_produced(),
            NodeType::PheromoneWrite(pheromone_write) => pheromone_write.inputs_produced(),
            NodeType::Thruster(thruster) => thruster.inputs_produced(),
        }
    }
}

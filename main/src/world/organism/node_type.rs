use std::f32::consts::PI;

use rand::{Rng, rngs::ThreadRng};
use serde::{Deserialize, Serialize};

use crate::{
    config::config::{Node as NodeConfig, Organism as OrganismConfig},
    consts::{KN, N, PHEROMONE_LAYERS},
    world::{
        environment::environment::Environment,
        organism::{
            in_out::OutputConsumedInputProduced,
            mutation::{Mut, Mutation},
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
    fn rand(rng: &mut ThreadRng, oc: &OrganismConfig, o: &Organism) -> Option<Self> {
        Some(match rng.random_range(0..=2) {
            0 => Self::Energy(Energy::new()),
            1 => Self::PheromoneRead(PheromoneRead::new(rng.random_range(0..PHEROMONE_LAYERS))),
            2 => Self::PheromoneWrite(PheromoneWrite::new(rng.random_range(0..PHEROMONE_LAYERS))),
            _ => Self::Thruster(Thruster::new(rng.random_range(-PI..PI))),
        })
    }
}
impl NodeType {
    pub fn out_con_in_prod(&self) -> OutputConsumedInputProduced {
        match self {
            NodeType::Energy(energy) => energy.in_out(),
            NodeType::PheromoneRead(pheromone_read) => pheromone_read.in_out(),
            NodeType::PheromoneWrite(pheromone_write) => pheromone_write.in_out(),
            NodeType::Thruster(thruster) => thruster.in_out(),
        }
    }
}

pub trait Node<A> {
    // Update the state and return the energy cost of doing so
    fn update_state(&mut self, node_config: NodeConfig, args: A) -> f32;

    // The number of brain outputs consumed by this node
    fn outputs_consumed(&self) -> usize;
    // The number of brain inputs produced by this node
    fn inputs_produced(&self) -> usize;

    fn in_out(&self) -> OutputConsumedInputProduced {
        return OutputConsumedInputProduced([self.outputs_consumed(), self.inputs_produced()]);
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Energy {
    collected_energy: f32,
}
impl Energy {
    pub fn new() -> Self {
        Self {
            collected_energy: 0.0,
        }
    }
}
impl Node<&mut Environment<N, KN>> for Energy {
    fn update_state(&mut self, node_config: NodeConfig, args: &mut Environment<N, KN>) -> f32 {
        todo!()
    }

    fn outputs_consumed(&self) -> usize {
        0
    }

    fn inputs_produced(&self) -> usize {
        0
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PheromoneRead {
    state: f32,
    layer_id: usize,
}
impl PheromoneRead {
    pub fn new(layer_id: usize) -> Self {
        Self {
            state: 0.0,
            layer_id,
        }
    }
}
impl Node<f32> for PheromoneRead {
    fn update_state(&mut self, node_config: NodeConfig, state: f32) -> f32 {
        let diff = (self.state - state).abs();
        self.state = state;

        return diff * node_config.pheromone_read_efficiency;
    }

    fn outputs_consumed(&self) -> usize {
        0
    }

    fn inputs_produced(&self) -> usize {
        1
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PheromoneWrite {
    state: f32,
    layer_id: usize,
}
impl PheromoneWrite {
    pub fn new(layer_id: usize) -> Self {
        Self {
            state: 0.0,
            layer_id,
        }
    }
}
impl Node<f32> for PheromoneWrite {
    fn update_state(&mut self, node_config: NodeConfig, state: f32) -> f32 {
        let diff = (self.state - state).abs();
        self.state = state;

        return diff * node_config.pheromone_write_efficiency;
    }

    fn outputs_consumed(&self) -> usize {
        1
    }

    fn inputs_produced(&self) -> usize {
        0
    }
}

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
impl Node<f32> for Thruster {
    fn update_state(&mut self, node_config: NodeConfig, state: f32) -> f32 {
        let diff = (self.state - state).abs();
        self.state = state;

        return diff * node_config.thruster_efficiency;
    }

    fn outputs_consumed(&self) -> usize {
        1
    }

    fn inputs_produced(&self) -> usize {
        0
    }
}

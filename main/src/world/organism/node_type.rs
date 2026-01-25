use std::f32::consts::PI;

use rand::{Rng, rngs::ThreadRng};

use crate::{
    config::config::{Node as NodeConfig, Organism as OrganismConfig},
    consts::{KN, N, PHEROMONE_LAYERS},
    world::{
        environment::environment::Environment,
        organism::{
            mutation::{Mut, Mutation},
            organism::Organism,
        },
    },
};

#[derive(Clone, Copy, Debug)]
pub enum NodeType {
    Energy(Energy),
    PheromoneRead(PheromoneRead),
    PheromoneWrite(PheromoneWrite),
    Thruster(Thruster),
}
impl Mut for NodeType {
    fn gen_mutation(rng: &mut ThreadRng, oc: &OrganismConfig, o: &Organism) -> Option<Self> {
        Some(match rng.gen_range(0..=2) {
            0 => Self::Energy(Energy::new()),
            1 => Self::PheromoneRead(PheromoneRead::new(rng.gen_range(0..PHEROMONE_LAYERS))),
            2 => Self::PheromoneWrite(PheromoneWrite::new(rng.gen_range(0..PHEROMONE_LAYERS))),
            _ => Self::Thruster(Thruster::new(rng.gen_range(-PI..PI))),
        })
    }
}
impl NodeType {
    // pub fn get_input(&self) -> Option<f32> {}
}

pub trait Node<A> {
    // Update the state and return the energy cost of doing so
    fn update_state(&mut self, node_config: NodeConfig, args: A) -> f32;
    fn get_in_out(&self) -> [usize; 2];
}

#[derive(Clone, Copy, Debug)]
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

    fn get_in_out(&self) -> [usize; 2] {
        return [0, 0];
    }
}

#[derive(Clone, Copy, Debug)]
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

    fn get_in_out(&self) -> [usize; 2] {
        todo!()
    }
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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

    fn get_in_out(&self) -> [usize; 2] {
        return [1, 1];
    }
}

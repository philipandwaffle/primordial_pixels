use rand::{Rng, rngs::ThreadRng};
use serde::{Deserialize, Serialize};

use crate::consts::PHEROMONE_LAYERS;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum LayerKey {
    Energy,
    Pheromone(usize),
    Decay,
}
impl LayerKey {
    pub fn rand_read_layer(rng: &mut ThreadRng) -> LayerKey {
        let id = rng.random_range(0..PHEROMONE_LAYERS + 1);
        if id == PHEROMONE_LAYERS {
            return LayerKey::Energy;
        } else {
            return LayerKey::Pheromone(id);
        }
    }
    pub fn rand_write_layer(rng: &mut ThreadRng) -> LayerKey {
        return LayerKey::Pheromone(rng.random_range(0..PHEROMONE_LAYERS));
    }

    pub fn next(&self) -> Self {
        match self {
            LayerKey::Energy => LayerKey::Pheromone(0),
            LayerKey::Pheromone(i) => match *i == PHEROMONE_LAYERS - 1 {
                true => LayerKey::Decay,
                false => LayerKey::Pheromone(i + 1),
            },
            LayerKey::Decay => LayerKey::Energy,
        }
    }
    pub fn prev(&self) -> Self {
        match self {
            LayerKey::Energy => LayerKey::Decay,
            LayerKey::Pheromone(i) => match *i == 0 {
                true => LayerKey::Energy,
                false => LayerKey::Pheromone(i - 1),
            },
            LayerKey::Decay => LayerKey::Pheromone(PHEROMONE_LAYERS - 1),
        }
    }
}

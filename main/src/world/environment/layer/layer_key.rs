use rand::{Rng, rngs::ThreadRng};
use serde::{Deserialize, Serialize};

use crate::consts::PHEROMONE_LAYERS;

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
pub enum LayerKey {
    Energy,
    // #[serde(rename = "pheromone_{n}")]
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

impl Serialize for LayerKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            LayerKey::Energy => "Energy".serialize(serializer),
            LayerKey::Decay => "Decay".serialize(serializer),
            LayerKey::Pheromone(n) => format!("Pheromone_{}", n).serialize(serializer),
        }
    }
}
impl<'de> Deserialize<'de> for LayerKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Energy" => Ok(LayerKey::Energy),
            "Decay" => Ok(LayerKey::Decay),
            s if s.starts_with("Pheromone_") => {
                let n_str = s.strip_prefix("Pheromone_").unwrap();
                n_str
                    .parse::<usize>()
                    .map(LayerKey::Pheromone)
                    .map_err(serde::de::Error::custom)
            }
            _ => Err(serde::de::Error::unknown_variant(&s, &["Energy", "Decay"])),
        }
    }
}

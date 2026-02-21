use std::collections::HashMap;

use bevy::math::Vec2;
use rand::{Rng, random_range, rngs::ThreadRng, seq::SliceRandom};

use crate::{
    config::config::Mutation as MutationConfig,
    consts::{JOINT_RADIUS, MAX_BONE_LEN, MIN_BONE_LEN},
    util::function::{rand_normal_vec2, rand_vec2, shuffled_indexes},
    world::organism::{
        distribution::Distribution, mutation::mutation::Mut, node_type::NodeType,
        organism::Organism,
    },
};

#[derive(Debug)]
pub enum Stats {
    MetronomeBeat { delta: f32 },
    IncubationPeriod { delta: f32 },
}
impl Stats {}
impl Mut for Stats {
    fn rand(rng: &mut ThreadRng, mutation_config: &MutationConfig, o: &Organism) -> Option<Self> {
        match random_range(0..=1) {
            0 => Some(Self::MetronomeBeat {
                delta: rng.random_range(-0.5..0.5),
            }),
            _ => Some(Self::IncubationPeriod {
                delta: rng.random_range(-0.5..0.5),
            }),
        }
    }
}

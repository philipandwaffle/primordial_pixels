use rand::{RngExt, rngs::ThreadRng};

use crate::{
    config::config::Mutation as MutationConfig,
    consts::{LOWER_SCALAR_MUTATION_BOUND, UPPER_SCALAR_MUTATION_BOUND},
    world::organism::{distribution::Distribution, mutation::mutation::Mut, organism::Organism},
};

#[derive(Debug)]
pub enum Stats {
    MetronomeBeat { delta: f32 },
    IncubationPeriod { delta: f32 },
}
impl Stats {}
impl Mut for Stats {
    fn rand(rng: &mut ThreadRng, mutation_config: &MutationConfig, _: &Organism) -> Option<Self> {
        match mutation_config.stats_distribution.get_index(rng) {
            0 => Some(Self::MetronomeBeat {
                delta: rng.random_range(LOWER_SCALAR_MUTATION_BOUND..UPPER_SCALAR_MUTATION_BOUND),
            }),
            _ => Some(Self::IncubationPeriod {
                delta: rng.random_range(LOWER_SCALAR_MUTATION_BOUND..UPPER_SCALAR_MUTATION_BOUND),
            }),
        }
    }
}

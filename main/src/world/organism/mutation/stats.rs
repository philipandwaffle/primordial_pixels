use rand::{Rng, rngs::ThreadRng};

use crate::{
    config::config::Mutation as MutationConfig,
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
                delta: rng.random_range(-0.5..0.5),
            }),
            _ => Some(Self::IncubationPeriod {
                delta: rng.random_range(-0.5..0.5),
            }),
        }
    }
}

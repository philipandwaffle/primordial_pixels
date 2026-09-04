use rand::rngs::ThreadRng;

use crate::{
    config::config::Mutation as MutationConfig,
    world::organism::{
        distribution::Distribution,
        mutation::{body::Body, brain::Brain, stats::Stats},
        organism::Organism,
    },
};

#[derive(Debug)]
pub enum Mutation {
    Body(Body),
    Brain(Brain),
    Stats(Stats),
}
impl Mut for Mutation {
    fn rand(rng: &mut ThreadRng, mutation_config: &MutationConfig, o: &Organism) -> Option<Self>
    where
        Self: Sized,
    {
        match mutation_config.type_distribution.get_index(rng) {
            0 => {
                if let Some(body_mutation) = Body::rand(rng, mutation_config, o) {
                    return Some(Mutation::Body(body_mutation));
                }
                None
            }
            _ => {
                if let Some(stats_mutation) = Stats::rand(rng, mutation_config, o) {
                    return Some(Mutation::Stats(stats_mutation));
                }
                None
            }
        }
    }
}

pub trait Mutable {
    fn mutate(&mut self, mutation: &Mutation) -> bool;
}

pub trait Mut {
    fn rand(rng: &mut ThreadRng, mutation_config: &MutationConfig, o: &Organism) -> Option<Self>
    where
        Self: Sized;
}

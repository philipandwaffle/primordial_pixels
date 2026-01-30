use rand::rngs::ThreadRng;

use crate::{
    config::config::Organism as OrganismConfig,
    world::organism::{
        mutation::{body::Body, brain::Brain},
        organism::Organism,
    },
};

#[derive(Debug)]
pub enum Mutation {
    Body(Body),
    Brain(Brain),
}
impl Mut for Mutation {
    fn rand(rng: &mut ThreadRng, oc: &OrganismConfig, o: &Organism) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(body_mutation) = Body::rand(rng, oc, o) {
            return Some(Mutation::Body(body_mutation));
        }
        None
    }
}

pub trait Mutable {
    fn mutate(&mut self, mutation: &Mutation) -> bool;
}

pub trait Mut {
    fn rand(rng: &mut ThreadRng, oc: &OrganismConfig, o: &Organism) -> Option<Self>
    where
        Self: Sized;
}

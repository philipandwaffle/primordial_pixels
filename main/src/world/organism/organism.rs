use bevy::math::Vec2;
use rand::rngs::ThreadRng;

use crate::world::organism::body::Body;
use crate::world::organism::brain::Brain;
use crate::world::organism::joint::Joint;
use crate::world::organism::mutation::{self, Mutable, Mutation};
use crate::world::organism::seed::Seed;

#[derive(Clone)]
pub struct Organism {
    pub brain: Option<Brain>,
    pub body: Body,
}
impl Mutable for Organism {
    fn mutate(&mut self, mutation: Mutation) -> bool {
        let mut o = self.clone();
        match mutation {
            Mutation::Body(body) => todo!(),
            Mutation::Brain(_) => {
                if let Some(b) = o.brain.as_mut() {
                    return b.mutate(mutation);
                }
            }
        }

        false
    }
}
impl Organism {
    pub fn new(brain: Option<Brain>, body: Body) -> Self {
        Self { brain, body }
    }

    pub fn as_seed(self, pos: Vec2) -> Seed {
        Seed::new(pos, self)
    }
}

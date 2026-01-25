use bevy::math::Vec2;
use rand::rngs::ThreadRng;

use crate::world::organism::joint::Joint;
use crate::world::organism::mutation::{self, Mutable, Mutation};

#[derive(Clone)]
pub struct Body {
    pub joints: Vec<Joint>,
    pub bones: Vec<[usize; 2]>,
    pub muscles: Vec<[usize; 2]>,
}
impl Mutable for Body {
    fn mutate(&mut self, mutation: Mutation) -> bool {
        let mut o = self.clone();
        match mutation {
            Mutation::Body(_) => todo!(),
            _ => unreachable!("Tried to mutate body using invalid mutation {:?}", mutation),
        }

        false
    }
}
impl Body {
    pub fn new(joints: Vec<Joint>, bones: Vec<[usize; 2]>, muscles: Vec<[usize; 2]>) -> Self {
        Self {
            joints,
            bones,
            muscles,
        }
    }
}

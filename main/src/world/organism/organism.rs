use bevy::{core_pipeline::core_2d::graph::input, math::Vec2};

use crate::world::organism::{
    body::Body,
    brain::Brain,
    mutation::{Mutable, Mutation},
    seed::Seed,
    stats::StaticStats,
};

#[derive(Clone)]
pub struct Organism {
    pub brain: Option<Brain>,
    pub body: Body,
    static_stats: StaticStats,
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
        Self {
            brain,
            body,
            static_stats: StaticStats::new(1.0),
        }
    }

    pub fn get_static_stats<'a>(&'a self) -> &'a StaticStats {
        return &self.static_stats;
    }

    pub fn as_seed(self, pos: Vec2) -> Seed {
        Seed::new(pos, self)
    }

    pub fn get_node_inputs(&self, node_index: usize) -> usize {
        let mut inputs = 0;
        for i in 0..node_index {
            inputs += self.body.joints[i]
                .nodes
                .iter()
                .map(|n| n.get_in())
                .sum::<usize>()
        }
        inputs
    }

    pub fn get_node_outputs(&self, node_index: usize) -> usize {
        let mut outputs = 0;
        for i in 0..node_index {
            outputs += self.body.joints[i]
                .nodes
                .iter()
                .map(|n| n.get_out())
                .sum::<usize>()
        }
        outputs
    }
}

use bevy::log::info;
use rand::rngs::ThreadRng;

use crate::{
    config::config::Mutation as MutationConfig,
    world::organism::{
        mutation::mutation::{Mut, Mutation},
        organism::Organism,
    },
};

#[derive(Debug)]
pub enum Brain {
    AddInput { index: usize },
    RemoveInput { index: usize },
    AddOutput { index: usize },
    RemoveOutput { index: usize },
    Learn { learn_rate: f32, learn_factor: f32 },
}
impl Mut for Brain {
    fn rand(_: &mut ThreadRng, mutation_config: &MutationConfig, _: &Organism) -> Option<Self> {
        Some(Brain::Learn {
            learn_rate: mutation_config.learn_rate,
            learn_factor: mutation_config.learn_factor,
        })
    }
}
impl Brain {
    pub fn from_in_out(
        [out_con_offset, in_prod_offset]: [usize; 2],
        [out_con, in_prod]: [usize; 2],
        add: bool,
    ) -> Vec<Mutation> {
        let mut res: Vec<_> = vec![];

        // Alter output neurones
        let out_range = 0..out_con;
        if add {
            for i in out_range {
                let i = out_con_offset + i;
                // info!("output {i}");
                res.push(Mutation::Brain(Brain::AddOutput { index: i }));
            }
        } else {
            for i in out_range.rev() {
                let i = out_con_offset + i;
                // info!("output {i}");
                res.push(Mutation::Brain(Brain::RemoveOutput { index: i }));
            }
        }

        // Alter input neurones
        let in_range = 0..in_prod;
        if add {
            for i in in_range {
                let i = in_prod_offset + i;
                // info!("input {i}");
                res.push(Mutation::Brain(Brain::AddInput { index: i }));
            }
        } else {
            for i in in_range.rev() {
                let i = in_prod_offset + i;
                // info!("input {i}");
                res.push(Mutation::Brain(Brain::RemoveInput { index: i }));
            }
        }
        res
    }

    pub fn is_learn(&self) -> bool {
        if let Brain::Learn {
            learn_rate: _,
            learn_factor: _,
        } = self
        {
            true
        } else {
            false
        }
    }
}

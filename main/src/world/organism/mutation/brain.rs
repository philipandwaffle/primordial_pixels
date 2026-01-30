use rand::rngs::ThreadRng;

use crate::{
    config::config::Organism as OrganismConfig,
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
    fn rand(_: &mut ThreadRng, oc: &OrganismConfig, _: &Organism) -> Option<Self> {
        Some(Brain::Learn {
            learn_rate: oc.learn_rate,
            learn_factor: oc.learn_factor,
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
        for i in 0..out_con {
            let i = out_con_offset + i;

            if add {
                res.push(Mutation::Brain(Brain::AddInput { index: i }));
            } else {
                res.push(Mutation::Brain(Brain::RemoveInput { index: i - 1 }));
            }
        }

        // remove output neurones for node
        for i in 0..in_prod {
            let i = in_prod_offset + i;

            if add {
                res.push(Mutation::Brain(Brain::AddOutput { index: i }));
            } else {
                res.push(Mutation::Brain(Brain::RemoveOutput { index: i - 1 }));
            }
        }
        res
    }
}

use std::collections::HashMap;

use bevy::ecs::resource::Resource;
use uuid::Uuid;

use crate::{consts::ENV_SIDE_CELLS, world::organism::seed::Seed};

#[derive(Resource)]
pub struct PetriDishInfo {
    pub init_seed: Seed,
    pub cur_organisms: usize,
    pub min_organisms: usize,
    pub initial_num_mutations: usize,
    pub num_mutations: usize,
    pub threshold: f32,
    pub side_len: f32,
    pub half_side_len: f32,
}
impl PetriDishInfo {
    pub fn new(
        init_seed: Option<Seed>,
        min_organisms: usize,
        num_mutations: usize,
        initial_num_mutations: usize,
        side_len: f32,
    ) -> Self {
        Self {
            init_seed: init_seed.unwrap_or(Seed::default()),
            cur_organisms: 0,
            min_organisms,
            initial_num_mutations,
            num_mutations,
            threshold: side_len * 0.5 * ((ENV_SIDE_CELLS - 1) as f32 / ENV_SIDE_CELLS as f32),
            side_len,
            half_side_len: side_len * 0.5,
        }
    }
}

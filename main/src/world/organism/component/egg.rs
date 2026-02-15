use bevy::ecs::{component::Component, message::MessageWriter};

use crate::world::organism::{message::SpawnOrganismMsg, organism::Organism};

#[derive(Component)]
pub struct Egg {
    pub total_time: f32,
    pub time_left: f32,
    pub organism: Organism,
}
impl Egg {
    pub fn new(total_time: f32, organism: Organism) -> Self {
        Self {
            total_time,
            time_left: total_time,
            organism,
        }
    }

    pub fn tick(&mut self, dt: f32, spawn_seed_msg: &mut MessageWriter<SpawnOrganismMsg>) {}
}

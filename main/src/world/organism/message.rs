use bevy::ecs::{entity::Entity, message::Message, system::Commands};

use crate::{assets::handles::Handles, world::organism::seed::Seed};

#[derive(Message)]
pub struct SpawnSeedMsg {
    seed: Seed,
}
impl SpawnSeedMsg {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn spawn(&self, commands: &mut Commands, h: &Handles) -> Entity {
        self.seed.spawn(commands, h)
    }
}

use bevy::ecs::message::Message;

use crate::world::organism::seed::Seed;

#[derive(Message)]
pub struct SpawnSeedMsg {
    pub seed: Seed,
}

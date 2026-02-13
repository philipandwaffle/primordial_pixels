use bevy::ecs::message::Message;

use crate::world::organism::seed::Seed;

#[derive(Message)]
pub struct LogOrganismsMsg {
    pub(crate) seeds: Vec<Seed>,
    pub(crate) path: String,
}
impl LogOrganismsMsg {
    pub fn new(seeds: Vec<Seed>, path: String) -> Self {
        return Self { seeds, path };
    }
}

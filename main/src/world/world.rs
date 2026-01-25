use bevy::ecs::entity::Entity;

use crate::world::environment::environment::Environment;

pub struct World<const N: usize, const KN: usize> {
    organisms: Vec<Entity>,
    environment: Environment<N, KN>,
}

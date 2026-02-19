use bevy::{
    ecs::{component::Component, message::MessageWriter},
    math::{Vec2, vec2},
    transform::components::Transform,
};

use crate::{
    consts::MIN_EGG_RADIUS,
    util::ticker::Ticker,
    world::organism::{message::SpawnOrganismMsg, organism::Organism},
};

#[derive(Component)]
pub struct Egg {
    ticker: Ticker,
    pub organism: Organism,
}
impl Egg {
    pub fn new(incubation_period: f32, organism: Organism) -> Self {
        Self {
            ticker: Ticker::new(incubation_period),
            organism,
        }
    }

    pub fn tick(
        &mut self,
        dt: f32,
        trans: &mut Transform,
        spawn_seed_msg: &mut MessageWriter<SpawnOrganismMsg>,
    ) -> bool {
        let radius = (self.organism.meta.radius * (self.ticker.elapsed / self.ticker.interval))
            .max(MIN_EGG_RADIUS);
        trans.scale = vec2(radius, radius).extend(1.0);

        if self.ticker.apply_dt(dt) {
            spawn_seed_msg.write(SpawnOrganismMsg::new(
                trans.translation.truncate(),
                self.organism.clone(),
            ));
            return true;
        }
        return false;
    }
}

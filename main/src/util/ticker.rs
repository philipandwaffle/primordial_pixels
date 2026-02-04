use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct Ticker {
    elapsed: f32,
    interval: f32,
}
impl Ticker {
    pub fn new(interval: f32) -> Self {
        Self {
            elapsed: 0.0,
            interval,
        }
    }

    pub fn apply_dt(&mut self, dt: f32) -> bool {
        self.elapsed += dt;
        if self.elapsed > self.interval {
            self.elapsed -= self.interval;
            return true;
        }

        return false;
    }
}

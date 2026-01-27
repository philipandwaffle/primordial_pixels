use avian2d::prelude::LinearDamping;
use bevy::{
    app::{Plugin, Update},
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        system::{Commands, Query, Res},
    },
    log::{info, trace},
    time::Time,
};

pub struct PhysicsLockPlugin;
impl Plugin for PhysicsLockPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, Self::update_locks);
    }
}
impl PhysicsLockPlugin {
    pub fn update_locks(
        mut commands: Commands,
        time: Res<Time>,
        mut locks: Query<(Entity, &mut LinearDamping, &mut PhysicsLock)>,
    ) {
        let dt = time.delta_secs();
        for (ent, mut damping, mut lock) in locks.iter_mut() {
            let new_mass = lock.tick(dt);
            if new_mass == 0.0 {
                commands
                    .entity(ent)
                    .remove::<(PhysicsLock, LinearDamping)>();
                trace!("De-spawning mass lock");
                continue;
            }
            *damping = LinearDamping(new_mass);
        }
    }
}

#[derive(Bundle)]
pub struct PhysicsLockBundle {
    damping: LinearDamping,
    lock: PhysicsLock,
}
impl PhysicsLockBundle {
    pub fn new(total_time: f32, starting_damping: f32) -> Self {
        return Self {
            damping: LinearDamping(starting_damping),
            lock: PhysicsLock::new(total_time, starting_damping),
        };
    }
    fn spawn(self, commands: &mut Commands) -> Entity {
        return commands.spawn(self).id();
    }
}

#[derive(Component)]
pub struct PhysicsLock {
    total_time: f32,
    time_left: f32,
    starting_damping: f32,
}
impl PhysicsLock {
    pub fn new(total_time: f32, starting_damping: f32) -> Self {
        return Self {
            total_time: total_time,
            time_left: total_time,
            starting_damping,
        };
    }

    pub fn tick(&mut self, dt: f32) -> f32 {
        self.time_left -= dt;

        if self.time_left > 0.0 {
            return (self.starting_damping * (self.time_left / self.total_time)).powf(5.0);
        } else {
            return 0.0;
        }
    }
}

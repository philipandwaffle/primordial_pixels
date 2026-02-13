use avian2d::prelude::{Collider, RigidBody};
use bevy::{
    app::{Plugin, PostStartup, Startup, Update},
    ecs::{
        bundle::Bundle,
        system::{Commands, Res},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::{Vec3, vec2, vec3},
    mesh::Mesh2d,
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use crate::{
    assets::handles::{Handles, MatKey, MeshKey},
    config::plugin::load_config,
    world::organism::{body::Body, brain::Brain, joint::Joint, organism::Organism, seed::Seed},
};

#[derive(Default)]
struct SandboxRes {
    seed: Seed,
}

pub struct SandboxPlugin;
impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, Self::spawn_cage)
            .add_systems(Update, Self::spawn_balls);
    }
}
impl SandboxPlugin {
    fn spawn_cage(handles: Res<Handles>, mut commands: Commands) {
        commands.spawn((
            RigidBody::Static,
            handles.get_mesh2d(&MeshKey::Rectangle),
            handles.get_mat2d(&MatKey::White),
            Transform::default()
                .with_translation(vec3(0.0, -20.0, -1.0))
                .with_scale(vec3(1000.0, 5.0, 1.0)),
            Collider::rectangle(1.0, 1.0),
        ));

        commands.spawn((
            RigidBody::Static,
            handles.get_mesh2d(&MeshKey::Rectangle),
            handles.get_mat2d(&MatKey::White),
            Transform::default()
                .with_translation(vec3(-500.0, 0.0, -1.0))
                .with_scale(vec3(5.0, 400.0, 1.0)),
            Collider::rectangle(1.0, 1.0),
        ));

        commands.spawn((
            RigidBody::Static,
            handles.get_mesh2d(&MeshKey::Rectangle),
            handles.get_mat2d(&MatKey::White),
            Transform::default()
                .with_translation(vec3(500.0, 0.0, -1.0))
                .with_scale(vec3(5.0, 400.0, 1.0)),
            Collider::rectangle(1.0, 1.0),
        ));
    }

    fn spawn_balls(mut commands: Commands, keys: Res<ButtonInput<KeyCode>>, h: Res<Handles>) {
        if keys.just_released(KeyCode::Space) {
            let cfg = load_config();
            let s = Seed::new(
                vec2(10.0, 0.0),
                Organism::new(
                    Some(Brain::new(vec![2, 4, 1])),
                    Body::new(
                        vec![
                            Joint::new(vec2(-5.0, 0.0), vec![]),
                            Joint::new(vec2(0.0, 6.0), vec![]),
                            Joint::new(vec2(5.0, 0.0), vec![]),
                        ],
                        vec![[0, 1], [1, 2]],
                        vec![[0, 1]],
                    ),
                    cfg.organism.metabolism,
                ),
            );
            s.clone().spawn(&mut commands, &h);
            // OrganismSpawner::spawn(s, &mut commands, &h);
        }
    }
}

#[derive(Bundle, Clone)]
struct Ball {
    rb: RigidBody,
    mesh: Mesh2d,
    mat: MeshMaterial2d<ColorMaterial>,
    trans: Transform,
    col: Collider,
}
impl Ball {
    pub fn new(h: &Handles) -> Self {
        Self {
            rb: RigidBody::Dynamic,
            mesh: h.get_mesh2d(&MeshKey::Circle),
            mat: h.get_mat2d(&MatKey::Green),
            trans: Transform::default().with_translation(vec3(0.0, 0.0, -2.0)),
            col: Collider::circle(1.0),
        }
    }

    pub fn with_translation(mut self, trans: Vec3) -> Self {
        self.trans = self.trans.with_translation(trans);
        return self;
    }
}

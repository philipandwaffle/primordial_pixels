use avian2d::prelude::*;
use bevy::{
    prelude::*,
    window::{WindowMode, WindowResolution},
};

use crate::{
    assets::plugin::HandlesPlugin,
    camera::PanningCamPlugin,
    config::plugin::ConfigPlugin,
    organism_logger::SavePlugin,
    performance_info::plugin::PerformanceInfoPlugin,
    physics_lock::PhysicsLockPlugin,
    runner::plugin::RunnerPlugin,
    sandbox::SandboxPlugin,
    world::{
        organism::{body::Body, joint::Joint, organism::Organism, seed::Seed},
        plugin::WorldPlugin,
    },
};
mod assets;
mod camera;
mod config;
mod consts;
mod extension;
mod organism_logger;
mod performance_info;
mod petri_dish;
mod physics_lock;
mod runner;
mod sandbox;
mod util;
mod world;

fn main() {
    // return;
    let mut a = App::new();

    // Performance stats
    // a.add_plugins(PerformanceInfoPlugin);

    // Sandbox
    // a.add_plugins(SandboxPlugin);

    a.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            // primary_window: None,
            // exit_condition: bevy::window::ExitCondition::DontExit,
            primary_window: Some(Window {
                title: "primordial_pixels".into(),
                resolution: WindowResolution::new(2560 / 2, 1440),
                mode: WindowMode::Windowed,
                position: WindowPosition::At(IVec2::new(0, 0)),
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }),
        ConfigPlugin,
        HandlesPlugin,
        PanningCamPlugin,
        PhysicsPlugins::default(),
        // PhysicsDebugPlugin,
        PhysicsLockPlugin,
        WorldPlugin,
    ));

    a.run();
}

// fn setup(mut commands: Commands, handles: Res<Handles>) {
//     println!("hello");
//     commands.spawn((
//         RigidBody::Dynamic,
//         handles.get_mesh2d(&assets::handles::MeshKey::Circle),
//         handles.get_mat2d(&assets::handles::MatKey::Black),
//         Transform::from_xyz(0.0, 0.0, -10.0).with_scale(vec3(100.0, 100.0, 100.0)),
//         Collider::circle(0.5),
//     ));
//     // commands.spawn((
//     //     RigidBody::Dynamic,
//     //     handles.get_mesh2d(&assets::handles::MeshKey::Circle),
//     //     handles.get_mat2d(&assets::handles::MatKey::Black),
//     //     Transform::from_xyz(0.0, 10.0, 0.0),
//     //     Collider::circle(0.5),
//     // ));
// }

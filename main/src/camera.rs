use std::f32::INFINITY;

use bevy::{
    app::{Plugin, PostStartup, Startup, Update},
    camera::{Camera2d, Projection},
    ecs::{
        bundle::Bundle,
        component::Component,
        system::{Commands, Query, Res},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::Vec3,
    time::Time,
    transform::components::Transform,
};

use crate::config::config::Camera as CamCfg;

pub struct PanningCamPlugin;
impl Plugin for PanningCamPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, Self::spawn_cam)
            .add_systems(PostStartup, Self::set_cam_zoom)
            .add_systems(Update, Self::pan_cam);
    }
}
impl PanningCamPlugin {
    pub fn spawn_cam(cam_cfg: Res<CamCfg>, mut commands: Commands) {
        commands.spawn(PanningCamBundle::new(
            cam_cfg.start_zoom,
            cam_cfg.max_zoom,
            cam_cfg.min_zoom,
        ));
    }

    pub fn set_cam_zoom(mut cam: Query<(&mut Projection, &PanningCam)>) {
        if let Ok((mut p, pc)) = cam.single_mut() {
            if let Some(start_zoom) = pc.start_zoom {
                if let Projection::Orthographic(op) = p.as_mut() {
                    op.scale = start_zoom;
                }
            }
        }
    }

    pub fn pan_cam(
        keys: Res<ButtonInput<KeyCode>>,
        time: Res<Time>,
        cam_cfg: Res<CamCfg>,
        mut cam: Query<(&mut Transform, &mut Projection, &PanningCam)>,
    ) {
        if let Ok((mut trans, mut p, pc)) = cam.single_mut() {
            let dt = time.delta_secs();

            let mut dir = Vec3::ZERO;

            if let Projection::Orthographic(op) = p.as_mut() {
                let pan_modifier = op.scale.powf(-0.25);
                if keys.pressed(KeyCode::KeyW) {
                    dir.y += 1.0;
                }
                if keys.pressed(KeyCode::KeyA) {
                    dir.x -= 1.0;
                }
                if keys.pressed(KeyCode::KeyS) {
                    dir.y -= 1.0;
                }
                if keys.pressed(KeyCode::KeyD) {
                    dir.x += 1.0;
                }
                trans.translation += dir * pan_modifier * cam_cfg.move_speed * dt;

                let mut zoom = 0.0;
                if keys.pressed(KeyCode::ArrowUp) {
                    zoom -= 1.0;
                }
                if keys.pressed(KeyCode::ArrowDown) {
                    zoom += 1.0;
                }
                op.scale += zoom * cam_cfg.zoom_speed * dt;
                op.scale = op.scale.clamp(
                    match pc.min_zoom {
                        Some(min) => min,
                        None => f32::EPSILON,
                    },
                    match pc.max_zoom {
                        Some(max) => max,
                        None => INFINITY,
                    },
                );
            }
        }
    }
}

#[derive(Bundle)]
pub struct PanningCamBundle {
    camera2d: Camera2d,
    panning_cam: PanningCam,
}
impl PanningCamBundle {
    pub fn new(start_zoom: Option<f32>, max_zoom: Option<f32>, min_zoom: Option<f32>) -> Self {
        Self {
            camera2d: Default::default(),
            panning_cam: PanningCam::new(start_zoom, max_zoom, min_zoom),
        }
    }
}

#[derive(Component)]
pub struct PanningCam {
    start_zoom: Option<f32>,
    max_zoom: Option<f32>,
    min_zoom: Option<f32>,
    // panning_speed: f32,
    // zoom_speed: f32,
}
impl PanningCam {
    pub fn new(
        start_zoom: Option<f32>,
        max_zoom: Option<f32>,
        min_zoom: Option<f32>,
        // panning_speed: f32,
        // zoom_speed: f32,
    ) -> Self {
        Self {
            start_zoom,
            max_zoom,
            min_zoom,
            // panning_speed,
            // zoom_speed,
        }
    }
}

use bevy::{ecs::resource::Resource, math::Vec2};
use serde::{Deserialize, Serialize};

use crate::{config::config_tag::ConfigTag, consts::NUM_MUTATIONS, runner::plugin::RunnerPlugin};
use my_derive::ConfigTag;

#[derive(ConfigTag, Serialize, Deserialize, Clone, Resource)]
pub struct Config {
    pub performance_debug: bool,
    pub camera: Camera,
    pub environment: Environment,
    pub organism: Organism,
    pub node: Node,
    pub runner: Option<RunnerPlugin>,
}

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy, Resource)]
pub struct Camera {
    pub move_speed: f32,
    pub zoom_speed: f32,
    pub start_zoom: Option<f32>,
    pub min_zoom: Option<f32>,
    pub max_zoom: Option<f32>,
}

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy, Resource)]
pub struct Environment {
    pub size: Vec2,
    pub display_update_interval: f32,
}

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy, Resource)]
pub struct Organism {
    pub muscle_efficiency: f32,
    pub mutation_rate: f32,
    pub mutation_distribution: [f32; NUM_MUTATIONS],
    pub learn_rate: f32,
    pub learn_factor: f32,
}

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy, Resource)]
pub struct Node {
    pub energy_collect_rate: f32,
    pub pheromone_read_efficiency: f32,
    pub pheromone_write_efficiency: f32,
    pub thruster_efficiency: f32,
}

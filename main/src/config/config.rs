use bevy::ecs::resource::Resource;
use serde::{Deserialize, Serialize};

use crate::config::config_tag::ConfigTag;
use my_derive::ConfigTag;

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy, Resource)]
pub struct Config {
    pub camera: Camera,
    pub organism: Organism,
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
pub struct Organism {
    pub energy_collect_rate: f32,
    pub muscle_efficiency: f32,
    pub learn_rate: f32,
    pub learn_factor: f32,
    pub node: Node,
}

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy, Resource)]
pub struct Node {
    pub energy_collect_rate: f32,
    pub pheromone_read_efficiency: f32,
    pub pheromone_write_efficiency: f32,
    pub thruster_efficiency: f32,
}

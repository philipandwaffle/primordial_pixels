use bevy::{ecs::resource::Resource, math::Vec2};
use serde::{Deserialize, Serialize};

use crate::{
    config::config_tag::ConfigTag,
    consts::NUM_MUTATIONS,
    petri_dish::plugin::PetriDishPlugin,
    // runner::plugin::RunnerPlugin,
    save::plugin::SavePlugin,
};
use my_derive::ConfigTag;

#[derive(ConfigTag, Serialize, Deserialize, Clone, Resource)]
pub struct Config {
    pub performance_debug: bool,
    pub camera: Camera,
    pub organism: Organism,
    pub physics: Physics,
    pub save: SavePlugin,
    // pub runner: Option<RunnerPlugin>,
    pub petri_dish: Option<PetriDishPlugin>,
}

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy, Resource)]
pub struct Camera {
    pub move_speed: f32,
    pub zoom_speed: f32,
    pub start_zoom: Option<f32>,
    pub min_zoom: Option<f32>,
    pub max_zoom: Option<f32>,
}

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy)]
pub struct Organism {
    pub mutation: Mutation,
    pub metabolism: Metabolism,
    pub transput: Transput,
}

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy, Resource)]
pub struct Mutation {
    pub rate: f32,
    pub distribution: [f32; NUM_MUTATIONS],
    pub learn_rate: f32,
    pub learn_factor: f32,
}

#[derive(ConfigTag, Default, Serialize, Deserialize, Clone, Copy, Resource)]
pub struct Metabolism {
    pub reproduce_threshold: f32,
    pub reproduce_cost: f32,
    pub node: f32,
    pub joint: f32,
    pub bone: f32,
    pub muscle: f32,
}

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy, Resource)]
pub struct Transput {
    pub muscle_efficiency: f32,
    pub energy_collect_rate: f32,
    pub pheromone_read_efficiency: f32,
    pub pheromone_write_efficiency: f32,
    pub thruster_strength: f32,
    pub thruster_efficiency: f32,
}

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy, Resource)]
pub struct Physics {
    pub gravity_scale: Vec2,
}

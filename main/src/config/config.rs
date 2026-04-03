use avian2d::parry::utils::hashmap::HashMap;
use bevy::{
    ecs::resource::Resource,
    math::{Vec2, usize},
};
use serde::{Deserialize, Serialize};

use crate::{
    config::config_tag::ConfigTag,
    consts::NUM_BODY_MUTATIONS,
    petri_dish::plugin::PetriDishPlugin,
    // runner::plugin::RunnerPlugin,
    save::plugin::SavePlugin,
    world::environment::{field::Field, layer::layer_key::LayerKey},
};
use my_derive::ConfigTag;

#[derive(ConfigTag, Serialize, Deserialize, Clone, Resource)]
pub struct Config<const KN: usize> {
    pub debug: Debug,
    pub camera: Camera,
    pub environment: Environment<KN>,
    pub organism: Organism,
    pub physics: Physics,
    pub save: SavePlugin,
    // pub runner: Option<RunnerPlugin>,
    pub petri_dish: Option<PetriDishPlugin>,
}

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy, Resource)]
pub struct Debug {
    pub physics: bool,
    pub performance: bool,
}

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy, Resource)]
pub struct Camera {
    pub move_speed: f32,
    pub zoom_speed: f32,
    pub start_zoom: Option<f32>,
    pub min_zoom: Option<f32>,
    pub max_zoom: Option<f32>,
}

#[derive(ConfigTag, Serialize, Deserialize, Clone, Resource)]
pub struct Environment<const KN: usize> {
    pub side_len: f32,
    pub layers: HashMap<LayerKey, Layer<KN>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Layer<const KN: usize> {
    Energy {
        kernel: Field<f32, KN>,
        permeability: f32,
        max: f32,
        rate: f32,
        on_interval: f32,
        off_interval: f32,
    },
    Decompose {
        kernel: Field<f32, KN>,
        initial_value: f32,
        permeability: f32,
        max: f32,
    },
    Pheromone {
        kernel: Field<f32, KN>,
        permeability: f32,
        max: f32,
    },
}

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy)]
pub struct Organism {
    pub mutation: Mutation,
    pub metabolism: Metabolism,
    pub storage: Storage,
    pub transput: Transput,
}

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy, Resource)]
pub struct Mutation {
    pub rate: f32,
    pub distribution: [f32; NUM_BODY_MUTATIONS],
    pub learn_rate: f32,
    pub learn_factor: f32,
}

#[derive(ConfigTag, Default, Serialize, Deserialize, Clone, Copy, Resource)]
pub struct Metabolism {
    pub decay_multiplier: f32,
    pub reproduce_threshold: f32,
    pub reproduce_cost: f32,
    pub node: f32,
    pub joint: f32,
    pub bone: f32,
    pub muscle: f32,
}

#[derive(ConfigTag, Default, Serialize, Deserialize, Clone, Copy, Resource)]
pub struct Storage {
    pub node: f32,
    pub joint: f32,
    pub bone: f32,
    pub muscle: f32,
}

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy, Resource)]
pub struct Transput {
    pub muscle_efficiency: f32,
    pub energy_collect_rate: f32,
    pub decomposer_collect_rate: f32,
    pub pheromone_read_efficiency: f32,
    pub pheromone_write_efficiency: f32,
    pub thruster_strength: f32,
    pub thruster_efficiency: f32,
    pub spike_collect_rate: f32,
    pub spike_collect_efficiency: f32,
}

#[derive(ConfigTag, Serialize, Deserialize, Clone, Copy, Resource)]
pub struct Physics {
    pub gravity_scale: Vec2,
}

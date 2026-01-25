pub const CONFIG_PATH: &'static str = "config.json";

// Organism sizes
pub const JOINT_RADIUS: f32 = 0.5;
pub const BONE_WIDTH: f32 = 0.25;
pub const MUSCLE_WIDTH: f32 = 0.25;

// Organism Z offset
pub const ORGANISM_Z: f32 = 0.0;
pub const JOINT_Z: f32 = ORGANISM_Z - 0.1;
pub const BONE_Z: f32 = ORGANISM_Z - 0.2;
pub const MUSCLE_Z: f32 = ORGANISM_Z - 0.3;

pub const MIN_MUSCLE_LEN: f32 = 0.5;
pub const MAX_MUSCLE_LEN: f32 = 1.5;

pub const NUM_BODY_MUTATIONS: usize = 2;
pub const NUM_BRAIN_MUTATIONS: usize = 2;

// Environment
pub const N: usize = 100;
pub const KN: usize = 9;

pub const PHEROMONE_LAYERS: usize = 8;

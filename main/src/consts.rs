pub const CONFIG_PATH: &'static str = "config.json";
// Z offsets
pub const ORGANISM_Z: f32 = 0.0;
pub const JOINT_Z: f32 = ORGANISM_Z - 0.1;
pub const BONE_Z: f32 = ORGANISM_Z - 0.2;
pub const MUSCLE_Z: f32 = ORGANISM_Z - 0.3;

pub const DISPLAY_Z: f32 = -1.0;

// Physics lock
pub const PHYS_LOCK_DUR: f32 = 0.5;
pub const PHYS_LOCK_DAMP: f32 = 50.0;

// Organism sizes
pub const JOINT_RADIUS: f32 = 0.5;
pub const BONE_WIDTH: f32 = 0.5;
pub const MUSCLE_WIDTH: f32 = 0.4;

// Energy
pub const JOINT_MAX_ENERGY: f32 = 5.0;

// Muscle
pub const MIN_MUSCLE_LEN: f32 = 0.7;
pub const MAX_MUSCLE_LEN: f32 = 1.3;
pub const MUSCLE_COMPLIANCE: f32 = 0.001;

// Mutation
pub const NUM_MUTATIONS: usize = 9;
pub const NUM_BODY_MUTATIONS: usize = 2;
pub const NUM_BRAIN_MUTATIONS: usize = 2;

pub const MIN_BONE_LEN: f32 = JOINT_RADIUS * 2.5;
pub const MAX_BONE_LEN: f32 = JOINT_RADIUS * 2.0 * 5.0;

// Brain
const MEMORY: usize = 0;
pub const BASE_INPUT: usize = MEMORY + 1;
pub const BASE_OUTPUT: usize = MEMORY + 0;

// 3 slots for random stuff,
pub const BASE_BRAIN_STRUCTURE: [usize; 6] = [BASE_INPUT, 8, 8, 8, 8, BASE_OUTPUT];

pub const MUSCLE_IN_PRODUCE: usize = 2;
pub const MUSCLE_OUT_CONSUME: usize = 1;

// Environment
pub const ENV_SIDE_LEN: usize = 100;
pub const N: usize = ENV_SIDE_LEN * ENV_SIDE_LEN;
pub const KN: usize = 9;

pub const PHEROMONE_LAYERS: usize = 8;

// Environment display
pub const NUM_COLORS: usize = 8;

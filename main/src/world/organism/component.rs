use bevy::{
    ecs::{component::Component, entity::Entity},
    log::error,
};

use crate::{
    consts::{MAX_MUSCLE_LEN, MIN_MUSCLE_LEN},
    world::organism::{brain::Brain, node_type::NodeType},
};

#[derive(Component)]
pub struct Organism {
    pub brain: Option<Brain>,
    pub joints: Vec<Entity>,
    pub bones: Vec<Entity>,
    pub muscle_bones: Vec<[usize; 2]>,
    pub muscles: Vec<Entity>,
}
impl Organism {
    pub fn new(
        brain: Option<Brain>,
        joints: Vec<Entity>,
        bones: Vec<Entity>,
        muscle_bones: Vec<[usize; 2]>,
        muscles: Vec<Entity>,
    ) -> Self {
        Self {
            brain,
            joints,
            bones,
            muscle_bones,
            muscles,
        }
    }
}

#[derive(Component)]
pub struct Joint {
    nodes: Vec<NodeType>,
}
impl Joint {
    pub fn new(nodes: &Vec<NodeType>) -> Self {
        Self {
            nodes: nodes.clone(),
        }
    }
}

#[derive(Component)]
pub struct Bone;
// {
//     pub base_z_rot: f32,
// }
impl Bone {
    // pub fn new(base_z_rot: f32) -> Self {
    //     Self { base_z_rot }
    // }
}

#[derive(Component)]
pub struct Muscle {
    cur_len: f32,
    rest_len: f32,
}
impl Muscle {
    pub fn new(rest_len: f32) -> Self {
        return Self {
            cur_len: 1.0,
            rest_len,
        };
    }

    pub fn set_len(&mut self, len: f32) -> f32 {
        if len > MIN_MUSCLE_LEN || len < MAX_MUSCLE_LEN {
            error!(
                "new muscle length is out of bounds, abort set. {} < {}  {}",
                MIN_MUSCLE_LEN, len, MAX_MUSCLE_LEN
            );
            return 0.0;
        }

        let abs_diff = (len - self.cur_len).abs();
        self.cur_len = len;

        abs_diff
    }

    pub fn get_absolute_len(&self) -> f32 {
        self.cur_len * self.rest_len
    }
}

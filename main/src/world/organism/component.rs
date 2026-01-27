use bevy::{
    asset::Handle,
    ecs::{component::Component, entity::Entity},
    log::error,
    math::Vec2,
    sprite_render::ColorMaterial,
};

use crate::{
    assets::handles::{Handles, MatKey},
    consts::{MAX_MUSCLE_LEN, MIN_MUSCLE_LEN},
    world::organism::{
        body::Body,
        brain::Brain,
        node_type::NodeType,
        organism::Organism,
        seed::Seed,
        stats::{StaticStats, VariableStats},
        util_trait::OrganismAccessor,
    },
};

#[derive(Component)]
pub struct OrganismEntity {
    pub organism: Organism,
    variable_stats: VariableStats,
    pub joint_ents: Vec<Entity>,
    pub bone_ents: Vec<Entity>,
    pub muscle_ents: Vec<Entity>,
}
impl OrganismEntity {
    pub fn new(
        organism: Organism,
        joints: Vec<Entity>,
        bones: Vec<Entity>,
        muscles: Vec<Entity>,
    ) -> Self {
        Self {
            organism,
            joint_ents: joints,
            bone_ents: bones,
            muscle_ents: muscles,
            variable_stats: VariableStats::new(),
        }
    }

    pub fn update_variable_stats(&mut self, dt: f32) {
        self.variable_stats.time_alive += dt;
    }

    pub fn get_static_stats<'a>(&'a self) -> &'a StaticStats {
        return self.organism.get_static_stats();
    }
    pub fn get_variable_stats<'a>(&'a self) -> &'a VariableStats {
        return &self.variable_stats;
    }

    pub fn as_seed(&self, pos: Vec2) -> Seed {
        self.organism.clone().as_seed(pos)
    }
}
impl OrganismAccessor for OrganismEntity {
    fn get_mut_organism<'a>(&'a mut self) -> &'a mut Organism {
        return &mut self.organism;
    }

    fn get_mut_body<'a>(&'a mut self) -> &'a mut Body {
        return &mut self.organism.body;
    }

    fn get_mut_brain<'a>(&'a mut self) -> Option<&'a mut Brain> {
        return self.organism.brain.as_mut();
    }

    fn get_organism<'a>(&'a self) -> &'a Organism {
        return &self.organism;
    }
    fn get_body<'a>(&'a self) -> &'a Body {
        return &self.organism.body;
    }

    fn get_brain<'a>(&'a self) -> &'a Option<Brain> {
        return &self.organism.brain;
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

    pub fn get_cur_len(&self) -> f32 {
        self.cur_len
    }

    pub fn set_len(&mut self, brain_out: f32) -> f32 {
        let abs_diff = (brain_out - self.cur_len).abs();
        self.cur_len = brain_out;

        abs_diff
    }

    pub fn get_mat(&self, h: &Handles) -> Handle<ColorMaterial> {
        if self.cur_len <= -0.6 {
            h.get_mat_handle(&MatKey::Red)
        } else if self.cur_len <= -0.2 {
            h.get_mat_handle(&MatKey::Crimson)
        } else if self.cur_len <= 0.2 {
            h.get_mat_handle(&MatKey::Magenta)
        } else if self.cur_len <= 0.6 {
            h.get_mat_handle(&MatKey::Purple)
        } else {
            h.get_mat_handle(&MatKey::Blue)
        }
    }

    pub fn get_absolute_len(&self) -> f32 {
        let scaled_len = 1.0 + self.cur_len * 0.5;
        ((MAX_MUSCLE_LEN - MIN_MUSCLE_LEN) * scaled_len + MIN_MUSCLE_LEN) * self.rest_len
    }
}

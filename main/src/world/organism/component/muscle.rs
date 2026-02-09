use std::collections::VecDeque;

use bevy::{
    asset::Handle,
    ecs::{
        component::Component,
        entity::Entity,
        query::{self, With},
        system::Query,
    },
    sprite_render::ColorMaterial,
    transform::components::Transform,
};

use crate::{
    assets::handles::{Handles, MatKey},
    config::config::Transput as TransputConfig,
    consts::{MAX_MUSCLE_LEN, MIN_MUSCLE_LEN},
    util::function::{quat_z_rot, rot_input},
    world::organism::{
        component::bone::Bone,
        transput::{Transput, append_input, remove_output},
    },
};

#[derive(Component)]
pub struct Muscle {
    bones: [Entity; 2],
    cur_len: f32,
    rest_len: f32,
}
impl Muscle {
    pub fn new(bones: [Entity; 2], rest_len: f32) -> Self {
        return Self {
            bones,
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
impl Transput<(), Query<'_, '_, &Transform, With<Bone>>> for Muscle {
    fn consume_outputs(
        &mut self,
        energy: &mut f32,
        out: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        _: (),
    ) {
        *energy += self.set_len(remove_output(out)) * transput_config.muscle_efficiency;
    }

    fn produce_inputs(
        &mut self,
        _: &mut f32,
        input: &mut VecDeque<f32>,
        _: &TransputConfig,
        bone_query: Query<&Transform, With<Bone>>,
    ) {
        if let Ok([trans_a, trans_b]) = bone_query.get_many(self.bones) {
            append_input(input, rot_input(quat_z_rot(trans_a.rotation)));
            append_input(input, rot_input(quat_z_rot(trans_b.rotation)));
        }
    }

    fn outputs_consumed(&self) -> usize {
        todo!()
    }

    fn inputs_produced(&self) -> usize {
        todo!()
    }
}

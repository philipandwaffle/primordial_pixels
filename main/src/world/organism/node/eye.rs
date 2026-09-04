use crate::{
    config::config::Transput as TransputConfig,
    consts::{EYE_MAX_FOV, EYE_MAX_RANGE, EYE_MIN_RANGE},
    util::function::rand_z_rot,
    world::organism::transput::{Transput, append_input},
};
use bevy::ecs::entity::hash_set::Iter;
use rand::{Rng, rngs::ThreadRng};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Eye {
    Eye1(GenericEye<1>),
    Eye2(GenericEye<2>),
    Eye3(GenericEye<3>),
    Eye4(GenericEye<4>),
    Eye5(GenericEye<5>),
}
impl Eye {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let z_rot = rand_z_rot(rng, 1.0);
        let ray_dist = rng.random_range(EYE_MIN_RANGE..=EYE_MAX_RANGE);
        let fov = rng.random_range(EYE_MAX_FOV..=EYE_MAX_FOV);

        match rng.random_range(0..=5) {
            1 => Self::Eye1(GenericEye::new(z_rot, ray_dist, fov)),
            2 => Self::Eye2(GenericEye::new(z_rot, ray_dist, fov)),
            3 => Self::Eye3(GenericEye::new(z_rot, ray_dist, fov)),
            4 => Self::Eye4(GenericEye::new(z_rot, ray_dist, fov)),
            _ => Self::Eye5(GenericEye::new(z_rot, ray_dist, fov)),
        }
    }
    pub fn get_num_rays(&self) -> usize {
        self.inputs_produced()
    }

    pub fn set_hit(&mut self, i: usize, val: f32) {
        match self {
            Eye::Eye1(e) => e.hits[i] = val,
            Eye::Eye2(e) => e.hits[i] = val,
            Eye::Eye3(e) => e.hits[i] = val,
            Eye::Eye4(e) => e.hits[i] = val,
            Eye::Eye5(e) => e.hits[i] = val,
        }
    }

    pub fn get_hits(&self) -> Vec<f32> {
        match self {
            Eye::Eye1(e) => e.hits.to_vec(),
            Eye::Eye2(e) => e.hits.to_vec(),
            Eye::Eye3(e) => e.hits.to_vec(),
            Eye::Eye4(e) => e.hits.to_vec(),
            Eye::Eye5(e) => e.hits.to_vec(),
        }
    }

    pub fn get_z_rot(&self) -> f32 {
        match self {
            Eye::Eye1(e) => e.z_rot,
            Eye::Eye2(e) => e.z_rot,
            Eye::Eye3(e) => e.z_rot,
            Eye::Eye4(e) => e.z_rot,
            Eye::Eye5(e) => e.z_rot,
        }
    }

    pub fn get_ray_dist(&self) -> f32 {
        match self {
            Eye::Eye1(e) => e.ray_dist,
            Eye::Eye2(e) => e.ray_dist,
            Eye::Eye3(e) => e.ray_dist,
            Eye::Eye4(e) => e.ray_dist,
            Eye::Eye5(e) => e.ray_dist,
        }
    }

    pub fn get_fov(&self) -> f32 {
        match self {
            Eye::Eye1(e) => e.fov,
            Eye::Eye2(e) => e.fov,
            Eye::Eye3(e) => e.fov,
            Eye::Eye4(e) => e.fov,
            Eye::Eye5(e) => e.fov,
        }
    }
    pub fn get_fov_mut(&mut self) -> &mut f32 {
        match self {
            Eye::Eye1(e) => &mut e.fov,
            Eye::Eye2(e) => &mut e.fov,
            Eye::Eye3(e) => &mut e.fov,
            Eye::Eye4(e) => &mut e.fov,
            Eye::Eye5(e) => &mut e.fov,
        }
    }
}
impl PartialEq for Eye {
    fn eq(&self, other: &Self) -> bool {
        return match (self, other) {
            (Eye::Eye1(a), Eye::Eye1(b)) => a == b,
            (Eye::Eye2(a), Eye::Eye2(b)) => a == b,
            (Eye::Eye3(a), Eye::Eye3(b)) => a == b,
            (Eye::Eye4(a), Eye::Eye4(b)) => a == b,
            (Eye::Eye5(a), Eye::Eye5(b)) => a == b,
            _ => false,
        };
    }
}
impl Transput<(), f32> for Eye {
    fn consume_outputs(
        &mut self,
        energy: &mut f32,
        output: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        args: (),
    ) {
        match self {
            Eye::Eye1(generic_eye) => {
                generic_eye.consume_outputs(energy, output, transput_config, args)
            }
            Eye::Eye2(generic_eye) => {
                generic_eye.consume_outputs(energy, output, transput_config, args)
            }
            Eye::Eye3(generic_eye) => {
                generic_eye.consume_outputs(energy, output, transput_config, args)
            }
            Eye::Eye4(generic_eye) => {
                generic_eye.consume_outputs(energy, output, transput_config, args)
            }
            Eye::Eye5(generic_eye) => {
                generic_eye.consume_outputs(energy, output, transput_config, args)
            }
        }
    }

    fn produce_inputs(
        &mut self,
        energy: &mut f32,
        input: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        dt: f32,
    ) {
        match self {
            Eye::Eye1(generic_eye) => {
                generic_eye.produce_inputs(energy, input, transput_config, dt)
            }
            Eye::Eye2(generic_eye) => {
                generic_eye.produce_inputs(energy, input, transput_config, dt)
            }
            Eye::Eye3(generic_eye) => {
                generic_eye.produce_inputs(energy, input, transput_config, dt)
            }
            Eye::Eye4(generic_eye) => {
                generic_eye.produce_inputs(energy, input, transput_config, dt)
            }
            Eye::Eye5(generic_eye) => {
                generic_eye.produce_inputs(energy, input, transput_config, dt)
            }
        }
    }

    fn outputs_consumed(&self) -> usize {
        match self {
            Eye::Eye1(generic_eye) => generic_eye.outputs_consumed(),
            Eye::Eye2(generic_eye) => generic_eye.outputs_consumed(),
            Eye::Eye3(generic_eye) => generic_eye.outputs_consumed(),
            Eye::Eye4(generic_eye) => generic_eye.outputs_consumed(),
            Eye::Eye5(generic_eye) => generic_eye.outputs_consumed(),
        }
    }

    fn inputs_produced(&self) -> usize {
        match self {
            Eye::Eye1(generic_eye) => generic_eye.inputs_produced(),
            Eye::Eye2(generic_eye) => generic_eye.inputs_produced(),
            Eye::Eye3(generic_eye) => generic_eye.inputs_produced(),
            Eye::Eye4(generic_eye) => generic_eye.inputs_produced(),
            Eye::Eye5(generic_eye) => generic_eye.inputs_produced(),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct GenericEye<const RAYS: usize> {
    #[serde(with = "BigArray")]
    pub hits: [f32; RAYS],
    z_rot: f32,
    ray_dist: f32,
    fov: f32,
    pub z_rot_offset: f32,
}
impl<const RAYS: usize> GenericEye<RAYS> {
    pub fn new(z_rot: f32, ray_dist: f32, fov: f32) -> Self {
        return Self {
            hits: [-1.0; RAYS],
            z_rot,
            ray_dist,
            fov,
            z_rot_offset: 0.0,
        };
    }
}
impl<const RAYS: usize> PartialEq for GenericEye<RAYS> {
    fn eq(&self, other: &Self) -> bool {
        return self.z_rot == other.z_rot
            && self.ray_dist == other.ray_dist
            && self.fov == other.fov;
    }
}
impl<const RAYS: usize> Transput<(), f32> for GenericEye<RAYS> {
    fn consume_outputs(&mut self, _: &mut f32, _: &mut VecDeque<f32>, _: &TransputConfig, _: ()) {}

    fn produce_inputs(
        &mut self,
        energy: &mut f32,
        input: &mut VecDeque<f32>,
        transput_config: &TransputConfig,
        dt: f32,
    ) {
        for hit in self.hits {
            append_input(input, if hit < 0.0 { 0.0 } else { hit / self.ray_dist });
        }

        *energy -= ((transput_config.eye_ray_efficiency * RAYS as f32)
            + (transput_config.eye_dist_efficiency * self.ray_dist))
            * dt;
    }

    fn outputs_consumed(&self) -> usize {
        0
    }

    fn inputs_produced(&self) -> usize {
        RAYS
    }
}

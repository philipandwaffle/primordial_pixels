use crate::{
    config::config::Transput as TransputConfig,
    consts::{EYE_MAX_FOV, EYE_MAX_RANGE, EYE_MIN_RANGE},
    util::function::{rand_z_rot, rot_output},
    world::organism::transput::{Transput, append_input, remove_output},
};
use rand::{RngExt, rngs::ThreadRng};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Eye {
    Eye1(GenericEye<1>),
    Eye3(GenericEye<3>),
    Eye5(GenericEye<5>),
    Eye7(GenericEye<7>),
}
impl Eye {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let z_rot = rand_z_rot(rng, 1.0);
        let ray_dist = rng.random_range(EYE_MIN_RANGE..=EYE_MAX_RANGE);
        let fov = rng.random_range(EYE_MAX_FOV..=EYE_MAX_FOV);

        match rng.random_range(0..=3) {
            0 => Self::Eye1(GenericEye::new(z_rot, ray_dist, fov)),
            1 => Self::Eye3(GenericEye::new(z_rot, ray_dist, fov)),
            2 => Self::Eye5(GenericEye::new(z_rot, ray_dist, fov)),
            _ => Self::Eye7(GenericEye::new(z_rot, ray_dist, fov)),
        }
    }
    pub fn get_num_rays(&self) -> usize {
        self.inputs_produced()
    }

    pub fn set_hit(&mut self, i: usize, mut new_val: f32) {
        if i <= self.get_num_rays() {
            // panic!("Set ")
        }
        if new_val >= 0.0 {
            let dist = self.get_ray_dist();
            new_val = new_val / dist;
        }

        match self {
            Eye::Eye1(e) => e.hits[i] = new_val,
            Eye::Eye3(e) => e.hits[i] = new_val,
            Eye::Eye5(e) => e.hits[i] = new_val,
            Eye::Eye7(e) => e.hits[i] = new_val,
        }
    }

    pub fn get_hits(&self) -> Vec<f32> {
        match self {
            Eye::Eye1(e) => e.hits.to_vec(),
            Eye::Eye3(e) => e.hits.to_vec(),
            Eye::Eye5(e) => e.hits.to_vec(),
            Eye::Eye7(e) => e.hits.to_vec(),
        }
    }

    pub fn get_z_rot(&self) -> f32 {
        match self {
            Eye::Eye1(e) => e.z_rot,
            Eye::Eye3(e) => e.z_rot,
            Eye::Eye5(e) => e.z_rot,
            Eye::Eye7(e) => e.z_rot,
        }
    }

    // pub fn get_z_rot_offset(&self) -> f32 {
    //     match self {
    //         Eye::Eye1(e) => e.z_rot_offset,
    //         Eye::Eye3(e) => e.z_rot_offset,
    //         Eye::Eye5(e) => e.z_rot_offset,
    //         Eye::Eye7(e) => e.z_rot_offset,
    //     }
    // }

    pub fn get_ray_dist(&self) -> f32 {
        match self {
            Eye::Eye1(e) => e.ray_dist,
            Eye::Eye3(e) => e.ray_dist,
            Eye::Eye5(e) => e.ray_dist,
            Eye::Eye7(e) => e.ray_dist,
        }
    }

    pub fn get_fov(&self) -> f32 {
        match self {
            Eye::Eye1(e) => e.fov,
            Eye::Eye3(e) => e.fov,
            Eye::Eye5(e) => e.fov,
            Eye::Eye7(e) => e.fov,
        }
    }
    pub fn get_fov_mut(&mut self) -> &mut f32 {
        match self {
            Eye::Eye1(e) => &mut e.fov,
            Eye::Eye3(e) => &mut e.fov,
            Eye::Eye5(e) => &mut e.fov,
            Eye::Eye7(e) => &mut e.fov,
        }
    }
}
impl PartialEq for Eye {
    fn eq(&self, other: &Self) -> bool {
        return match (self, other) {
            (Eye::Eye1(a), Eye::Eye1(b)) => a == b,
            (Eye::Eye3(a), Eye::Eye3(b)) => a == b,
            (Eye::Eye5(a), Eye::Eye5(b)) => a == b,
            (Eye::Eye7(a), Eye::Eye7(b)) => a == b,

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
            Eye::Eye3(generic_eye) => {
                generic_eye.consume_outputs(energy, output, transput_config, args)
            }
            Eye::Eye5(generic_eye) => {
                generic_eye.consume_outputs(energy, output, transput_config, args)
            }
            Eye::Eye7(generic_eye) => {
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
            Eye::Eye3(generic_eye) => {
                generic_eye.produce_inputs(energy, input, transput_config, dt)
            }
            Eye::Eye5(generic_eye) => {
                generic_eye.produce_inputs(energy, input, transput_config, dt)
            }
            Eye::Eye7(generic_eye) => {
                generic_eye.produce_inputs(energy, input, transput_config, dt)
            }
        }
    }

    fn outputs_consumed(&self) -> usize {
        match self {
            Eye::Eye1(generic_eye) => generic_eye.outputs_consumed(),
            Eye::Eye3(generic_eye) => generic_eye.outputs_consumed(),
            Eye::Eye5(generic_eye) => generic_eye.outputs_consumed(),
            Eye::Eye7(generic_eye) => generic_eye.outputs_consumed(),
        }
    }

    fn inputs_produced(&self) -> usize {
        match self {
            Eye::Eye1(generic_eye) => generic_eye.inputs_produced(),
            Eye::Eye3(generic_eye) => generic_eye.inputs_produced(),
            Eye::Eye5(generic_eye) => generic_eye.inputs_produced(),
            Eye::Eye7(generic_eye) => generic_eye.inputs_produced(),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct GenericEye<const RAYS: usize> {
    #[serde(with = "BigArray")]
    hits: [f32; RAYS],
    z_rot: f32,
    ray_dist: f32,
    fov: f32,
    // pub z_rot_offset: f32,
}
impl<const RAYS: usize> GenericEye<RAYS> {
    pub fn new(z_rot: f32, ray_dist: f32, fov: f32) -> Self {
        return Self {
            hits: [-1.0; RAYS],
            z_rot,
            ray_dist,
            fov,
            // z_rot_offset: 0.0,
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
    fn consume_outputs(&mut self, _: &mut f32, out: &mut VecDeque<f32>, _: &TransputConfig, _: ()) {
        // self.z_rot_offset = rot_output(remove_output(out));
    }

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

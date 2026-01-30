use std::f32::consts::PI;

use bevy::math::{Vec2, vec2};
use rand::{Rng, rngs::ThreadRng, seq::SliceRandom};

pub fn rand_normal_vec2(rng: &mut ThreadRng) -> Vec2 {
    let theta = rng.random::<f32>() * 2.0 * PI;
    return f32_to_vec(theta);
}

pub fn rand_vec2(rng: &mut ThreadRng, max_len: f32) -> Vec2 {
    return rand_normal_vec2(rng) * rng.random::<f32>() * max_len;
}

pub fn f32_to_vec(theta_radians: f32) -> Vec2 {
    return vec2(f32::sin(theta_radians), f32::cos(theta_radians));
}

pub fn shuffled_indexes(rng: &mut ThreadRng, len: usize) -> Vec<usize> {
    let mut vec = (0..len).collect::<Vec<usize>>();
    vec.shuffle(rng);
    vec
}

pub fn rot_input(input: f32) -> f32 {
    return input / (2.0 * PI);
}

pub fn clamp_out(out: f32) -> f32 {
    return 1.0 + out * 0.5;
}

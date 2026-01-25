use std::f32::consts::PI;

use bevy::math::{Vec2, vec2};
use rand::{Rng, rngs::ThreadRng, seq::SliceRandom};

pub fn rand_normal_vec2(rng: &mut ThreadRng) -> Vec2 {
    let theta = rng.r#gen::<f32>() * 2.0 * PI;
    return f32_to_vec(theta);
}

pub fn f32_to_vec(theta_radians: f32) -> Vec2 {
    return vec2(f32::sin(theta_radians), f32::cos(theta_radians));
}

pub fn shuffled_indexes(rng: &mut ThreadRng, len: usize) -> Vec<usize> {
    let mut vec = (0..len).collect::<Vec<usize>>();
    vec.shuffle(rng);
    vec
}

use bevy::{log::info, math::Vec2};
use rand::{Rng, rngs::ThreadRng};
use serde::{Deserialize, Serialize};

use crate::{
    consts::{NUM_BODY_MUTATIONS, NUM_BRAIN_MUTATIONS},
    util::function::rand_normal_vec2,
};

use super::distribution::Distribution;

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Chromosomes {
    pub spawn_offset: Chromosome<Vec2>,
    pub spawn_rot: Chromosome<f32>,
    pub body_mut_distribution: Chromosome<[f32; NUM_BODY_MUTATIONS]>,
    pub brain_mut_distribution: Chromosome<[f32; NUM_BRAIN_MUTATIONS]>,
    pub clock_mod: Chromosome<f32>,
    pub brain_mut_rate: Chromosome<f32>,
    pub brain_learn_rate: Chromosome<f32>,
    pub brain_learn_factor: Chromosome<f32>,
}
impl Chromosomes {
    pub fn new() -> Self {
        return Self {
            spawn_offset: Chromosome::new(Vec2::ZERO, 0.05, 2.5),
            spawn_rot: Chromosome::new(0.0, 0.05, 0.05),
            body_mut_distribution: Chromosome::new(
                [1.0 / NUM_BODY_MUTATIONS as f32; NUM_BODY_MUTATIONS],
                0.05,
                0.05,
            ),
            brain_mut_distribution: Chromosome::new(
                [1.0 / NUM_BRAIN_MUTATIONS as f32; NUM_BRAIN_MUTATIONS],
                0.05,
                0.05,
            ),
            clock_mod: Chromosome::new(5.0, 0.05, 0.05),
            brain_mut_rate: Chromosome::new(0.05, 0.05, 0.05),
            brain_learn_rate: Chromosome::new(0.02, 0.05, 0.05),
            brain_learn_factor: Chromosome::new(0.02, 0.05, 0.05),
        };
    }
    pub fn mutate(&mut self, rng: &mut ThreadRng) -> bool {
        let mut res = false;

        if self.spawn_offset.mutate(rng) {
            info!("Mutating spawn_offset chromosome");
            res = true;
        }
        if self.spawn_rot.mutate(rng) {
            info!("Mutating spawn_rot chromosome");
            res = true;
        }
        if self.body_mut_distribution.mutate(rng) {
            info!("Mutating body_mut_distribution chromosome");
            res = true;
        }
        if self.brain_mut_distribution.mutate(rng) {
            info!("Mutating brain_mut_distribution chromosome");
            res = true;
        }
        if self.clock_mod.mutate(rng) {
            info!("Mutating clock_mod chromosome");
            res = true;
        }
        if self.brain_learn_rate.mutate(rng) {
            info!("Mutating brain_mut_rate chromosome");
            res = true;
        }
        if self.brain_learn_factor.mutate(rng) {
            info!("Mutating brain_mut_factor chromosome");
            res = true;
        }

        return res;
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Chromosome<T> {
    pub val: T,
    rate: f32,
    factor: f32,
}
impl<T> Chromosome<T> {
    pub fn new(val: T, rate: f32, factor: f32) -> Self {
        return Self { val, rate, factor };
    }
}
impl Chromosome<f32> {
    fn mutate(&mut self, rng: &mut ThreadRng) -> bool {
        let should_mut = rng.random::<f32>() < self.rate;

        if should_mut {
            self.val += (1.0 - rng.random::<f32>()) * self.factor;
        }

        return should_mut;
    }
}
impl Chromosome<Vec2> {
    fn mutate(&mut self, rng: &mut ThreadRng) -> bool {
        let should_mut = rng.random::<f32>() < self.rate;

        if should_mut {
            self.val += rand_normal_vec2(rng) * self.factor;
        }

        return should_mut;
    }
}
impl<const NUM_TYPES: usize> Chromosome<[f32; NUM_TYPES]> {
    fn mutate(&mut self, rng: &mut ThreadRng) -> bool {
        let should_mut = rng.random::<f32>() < self.rate;

        if should_mut {
            let i = rng.random_range(0..NUM_TYPES);
            self.val[i] += rng.random::<f32>() * self.factor;
            self.val.normalise();
        }

        return should_mut;
    }
}

use std::ops::Index;

use bevy::math::FloatExt;
use serde::{Deserialize, Serialize};

use crate::world::environment::{accessor_trait::Env, field::Field};

#[derive(Debug, Serialize, Deserialize)]
pub struct Replenish<const N: usize> {
    field: Field<f32, N>,
    max: f32,
    rate: f32,
}
impl<const N: usize> Index<usize> for Replenish<N> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.field.space[index]
    }
}
impl<const N: usize> Replenish<N> {
    pub fn new(val: f32, max: f32, rate: f32) -> Self {
        Self {
            field: Field::<f32, N>::from_element(val),
            max,
            rate,
        }
    }
}
impl<const N: usize> Env<N> for Replenish<N> {
    fn field(&self) -> &Field<f32, N> {
        &self.field
    }

    fn field_mut(&mut self) -> &mut Field<f32, N> {
        &mut self.field
    }

    fn get(&self, x: isize, y: isize) -> f32 {
        self.field.get(x, y)
    }

    fn max(&self) -> f32 {
        self.max
    }

    fn update(&mut self, dt: f32) {
        self.replenish(dt)
    }
}
impl<const N: usize> ReplenishTrait<N> for Replenish<N> {
    fn rate(&self) -> f32 {
        self.rate
    }
}

pub trait ReplenishTrait<const N: usize>
where
    Self: Env<N>,
{
    fn rate(&self) -> f32;

    fn replenish(&mut self, dt: f32) {
        let l = self.field().side_len as isize;
        let m = self.max();
        let r = self.rate() * dt;

        for y in 0..l {
            for x in 0..l {
                let new_val = (self.field().get(x, y) + r).min(m);
                self.field_mut().set(x, y, new_val);
            }
        }
    }
}

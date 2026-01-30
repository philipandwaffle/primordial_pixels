use bevy::math::{FloatExt, Vec2};
use rand_distr::num_traits::Signed;
use serde::{Deserialize, Serialize};

use crate::world::environment::{accessor_trait::EnvAccessor, field::Field};

#[derive(Debug, Serialize, Deserialize)]
pub struct Replenish<const N: usize> {
    field: Field<N>,
    max: f32,
    rate: f32,
}
impl<const N: usize> EnvAccessor for Replenish<N> {
    fn get(&self, x: isize, y: isize) -> f32 {
        self.field.get(x, y)
    }

    fn delta(&mut self, x: isize, y: isize, delta: &mut f32) {
        let val = self.field.get_mut(x, y);
        let new_val = &*val - &*delta;

        if new_val.is_negative() {
            *delta -= new_val;
            *val = 0.0;
        } else {
            *delta = 0.0;
            *val = new_val.max(self.max);
        }
    }
}
impl<const N: usize> Replenish<N> {
    pub fn new(val: f32, max: f32, rate: f32) -> Self {
        Self {
            field: Field::<N>::from_element(val),
            max,
            rate,
        }
    }
    pub fn replenish(&mut self, dt: f32) {
        let l = self.field.side_len as isize;
        let r = self.rate * dt;

        for y in 0..l {
            for x in 0..l {
                let new_val = self.field.get(x, y).lerp(self.max, r);
                self.field.set(x, y, new_val);
            }
        }
    }
}

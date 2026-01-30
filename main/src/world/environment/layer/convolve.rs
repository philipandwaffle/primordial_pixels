use bevy::math::{FloatExt, Vec2};
use rand_distr::num_traits::Signed;
use serde::{Deserialize, Serialize};

use crate::world::environment::{accessor_trait::EnvAccessor, field::Field};

#[derive(Debug, Serialize, Deserialize)]
pub struct Convolve<const N: usize, const KN: usize> {
    field: Field<N>,
    kernel: Field<KN>,
    min: f32,
    max: f32,
}
impl<const N: usize, const KN: usize> EnvAccessor for Convolve<N, KN> {
    fn get(&self, x: isize, y: isize) -> f32 {
        self.field.get(x, y)
    }

    // fn set(&mut self, x: isize, y: isize, value: f32) {
    //     self.field.
    // }

    fn delta(&mut self, x: isize, y: isize, delta: &mut f32) {
        let new_val = self.get(x, y) - *delta;

        if new_val.is_negative() {
            *delta = -new_val;
            self.field.set(x, y, 0.0);
        } else {
            self.field.set(x, y, new_val);
        }
    }
}
impl<const N: usize, const KN: usize> Convolve<N, KN> {
    pub fn new(val: f32, kernel: Field<KN>, max: f32) -> Self {
        Self {
            field: Field::<N>::from_element(val),
            kernel,
            min: 0.0,
            max,
        }
    }

    pub fn with_field(mut self, field: Field<N>) -> Self {
        self.field = field;
        self
    }

    pub fn convolve(&mut self, dt: f32) {
        let l = self.field.side_len as isize;
        let kl = self.kernel.side_len as isize;
        let k_offset = -(kl / 2);

        let f = self.field;
        for y in 0..l {
            for x in 0..l {
                let mut new_val = 0.0;
                for ky in 0..kl {
                    for kx in 0..kl {
                        new_val += f.get(x + kx + k_offset, y + ky + k_offset)
                    }
                }

                new_val = self.field.get(x, y).lerp(new_val, dt);
                self.field.set(x, y, new_val);
            }
        }
    }
}

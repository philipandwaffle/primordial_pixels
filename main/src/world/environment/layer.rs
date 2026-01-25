use bevy::math::FloatExt;
use serde::{Deserialize, Serialize};

use crate::world::environment::field::Field;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplenishLayer<const N: usize> {
    field: Field<N>,
    max: f32,
    rate: f32,
}
impl<const N: usize> ReplenishLayer<N> {
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
                let new_val = self.field.get_value(x, y).lerp(self.max, r);
                self.field.set_value(x, y, new_val);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConvolveLayer<const N: usize, const KN: usize> {
    field: Field<N>,
    kernel: Field<KN>,
}
impl<const N: usize, const KN: usize> ConvolveLayer<N, KN> {
    pub fn new(val: f32, kernel: Field<KN>) -> Self {
        Self {
            field: Field::<N>::from_element(val),
            kernel,
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
                        new_val += f.get_value(x + kx + k_offset, y + ky + k_offset)
                    }
                }

                new_val = self.field.get_value(x, y).lerp(new_val, dt);
                self.field.set_value(x, y, new_val);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn y_coord_get_value() {
        let x = 3;
        println!("{}", x / 2);
    }
}

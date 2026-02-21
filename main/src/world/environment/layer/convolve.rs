use std::ops::{Index, IndexMut};

use bevy::math::FloatExt;
use serde::{Deserialize, Serialize};

use crate::world::environment::{accessor_trait::Env, field::Field};

#[derive(Debug, Serialize, Deserialize)]
pub struct Convolve<const N: usize, const KN: usize> {
    field: Field<f32, N>,
    kernel: Field<f32, KN>,
    permeability: f32,
    max: f32,
}
impl<const N: usize, const KN: usize> Convolve<N, KN> {
    pub fn new(val: f32, kernel: Field<f32, KN>, permeability: f32, max: f32) -> Self {
        Self {
            field: Field::<f32, N>::from_element(val),
            kernel,
            permeability,
            max,
        }
    }

    pub fn with_field(mut self, field: Field<f32, N>) -> Self {
        self.field = field;
        self
    }
}
impl<const N: usize, const KN: usize> Index<usize> for Convolve<N, KN> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.field.space[index]
    }
}
impl<const N: usize, const KN: usize> IndexMut<usize> for Convolve<N, KN> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.field[index]
    }
}
impl<const N: usize, const KN: usize> ConvolveTrait<N, KN> for Convolve<N, KN> {
    fn kernel(&self) -> Field<f32, KN> {
        self.kernel
    }

    fn permeability(&self) -> f32 {
        self.permeability
    }
}
impl<const N: usize, const KN: usize> Env<N> for Convolve<N, KN> {
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
        self.convolve(dt);
    }
}

pub trait ConvolveTrait<const N: usize, const KN: usize>
where
    Self: Env<N>,
{
    fn kernel(&self) -> Field<f32, KN>;
    fn permeability(&self) -> f32;

    fn convolve(&mut self, dt: f32) {
        let l = self.field().side_len as isize;
        let kl = self.kernel().side_len as isize;
        let k_offset = -(kl / 2);
        let inter = dt * self.permeability();

        let f = self.field().clone();
        for y in 0..l {
            for x in 0..l {
                let mut new_val = 0.0;
                for ky in 0..kl {
                    for kx in 0..kl {
                        new_val += f.get(x + kx + k_offset, y + ky + k_offset)
                    }
                }
                new_val /= KN as f32;

                new_val = self.field().get(x, y).lerp(new_val, inter);
                self.field_mut().set(x, y, new_val);
            }
        }
    }
}

use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

use crate::world::environment::{
    accessor_trait::Env,
    field::Field,
    layer::{convolve::ConvolveTrait, replenish::ReplenishTrait},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ReplenishConvolve<const N: usize, const KN: usize> {
    field: Field<f32, N>,
    kernel: Field<f32, KN>,
    permeability: f32,
    max: f32,
    rate: f32,
}
impl<const N: usize, const KN: usize> ReplenishConvolve<N, KN> {
    pub fn new(kernel: Field<f32, KN>, permeability: f32, max: f32, rate: f32) -> Self {
        Self {
            field: Field::<f32, N>::from_element(0.0),
            kernel,
            permeability,
            max,
            rate,
        }
    }
}
impl<const N: usize, const KN: usize> Index<usize> for ReplenishConvolve<N, KN> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.field[index]
    }
}
impl<const N: usize, const KN: usize> IndexMut<usize> for ReplenishConvolve<N, KN> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.field[index]
    }
}
impl<const N: usize, const KN: usize> Env<N> for ReplenishConvolve<N, KN> {
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
        self.replenish(dt);
        self.convolve(dt);
    }
}
impl<const N: usize, const KN: usize> ReplenishTrait<N> for ReplenishConvolve<N, KN> {
    fn rate(&self) -> f32 {
        self.rate
    }
}
impl<const N: usize, const KN: usize> ConvolveTrait<N, KN> for ReplenishConvolve<N, KN> {
    fn kernel(&self) -> Field<f32, KN> {
        self.kernel
    }

    fn permeability(&self) -> f32 {
        self.permeability
    }
}

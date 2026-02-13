use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

use crate::world::environment::{accessor_trait::Env, layer::convolve::Convolve};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplenishConvolve<const N: usize, const KN: usize> {
    convolve: Convolve<N, KN>,
    rate: f32,
}
impl<const N: usize, const KN: usize> Index<usize> for ReplenishConvolve<N, KN> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.convolve[index]
    }
}
impl<const N: usize, const KN: usize> IndexMut<usize> for ReplenishConvolve<N, KN> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.convolve[index]
    }
}
impl<const N: usize, const KN: usize> Env for ReplenishConvolve<N, KN> {
    fn get(&self, x: isize, y: isize) -> f32 {
        self.convolve.get(x, y)
    }

    // fn set(&mut self, x: isize, y: isize, value: f32) {
    //     self.field.
    // }

    fn delta(&mut self, x: isize, y: isize, delta: &mut f32) {
        self.convolve.delta(x, y, delta);
    }

    fn max(&self) -> f32 {
        self.convolve.max()
    }

    fn update(&mut self, dt: f32) {
        for i in 0..N {
            self[i] = (self[i] + self.rate * dt).min(self.max());
        }

        self.convolve.update(dt);
    }
}
impl<const N: usize, const KN: usize> ReplenishConvolve<N, KN> {
    pub fn new(convolve: Convolve<N, KN>, rate: f32) -> Self {
        Self { convolve, rate }
    }
}

use std::ops::{Index, IndexMut};

use rand_distr::num_traits::Signed;
use serde::{Deserialize, Serialize};

use crate::world::environment::{
    accessor_trait::Env,
    layer::{convolve::Convolve, replenish_convolve::ReplenishConvolve},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PeriodicReplenishConvolve<const N: usize, const KN: usize> {
    replenish_convolve: ReplenishConvolve<N, KN>,
    elapsed: f32,
    on_interval: f32,
    off_interval: f32,
    replenish: bool,
}
impl<const N: usize, const KN: usize> Index<usize> for PeriodicReplenishConvolve<N, KN> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.replenish_convolve[index]
    }
}
impl<const N: usize, const KN: usize> IndexMut<usize> for PeriodicReplenishConvolve<N, KN> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.replenish_convolve[index]
    }
}
impl<const N: usize, const KN: usize> Env for PeriodicReplenishConvolve<N, KN> {
    fn get(&self, x: isize, y: isize) -> f32 {
        self.replenish_convolve.get(x, y)
    }

    // fn set(&mut self, x: isize, y: isize, value: f32) {
    //     self.field.
    // }

    fn delta(&mut self, x: isize, y: isize, delta: &mut f32) {
        self.replenish_convolve.delta(x, y, delta);
    }

    fn max(&self) -> f32 {
        self.replenish_convolve.max()
    }

    fn update(&mut self, mut dt: f32) {
        self.elapsed += dt;

        let diff = match self.replenish {
            true => self.elapsed - self.on_interval,
            false => self.elapsed - self.off_interval,
        };

        if diff.is_positive() {
            self.replenish = !self.replenish;
            self.elapsed = 0.0;
            dt = diff;
        }

        if self.replenish {
            self.replenish_convolve.replenish(dt);
        }
        self.replenish_convolve.convolve.convolve(dt);
    }
}
impl<const N: usize, const KN: usize> PeriodicReplenishConvolve<N, KN> {
    pub fn new(
        replenish_convolve: ReplenishConvolve<N, KN>,
        on_interval: f32,
        off_interval: f32,
    ) -> Self {
        Self {
            replenish_convolve,
            elapsed: 0.0,
            on_interval,
            off_interval,
            replenish: true,
        }
    }
}

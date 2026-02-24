use std::ops::{Index, IndexMut};

use rand_distr::num_traits::Signed;
use serde::{Deserialize, Serialize};

use crate::world::environment::{
    accessor_trait::Env,
    field::Field,
    layer::{convolve::ConvolveTrait, replenish::ReplenishTrait},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PeriodicReplenishConvolve<const N: usize, const KN: usize> {
    field: Field<f32, N>,
    kernel: Field<f32, KN>,
    permeability: f32,
    max: f32,
    rate: f32,
    elapsed: f32,
    on_interval: f32,
    off_interval: f32,
    replenish: bool,
}
impl<const N: usize, const KN: usize> PeriodicReplenishConvolve<N, KN> {
    pub fn new(
        kernel: Field<f32, KN>,
        permeability: f32,
        max: f32,
        rate: f32,
        on_interval: f32,
        off_interval: f32,
    ) -> Self {
        Self {
            field: Field::<f32, N>::from_element(0.0),
            kernel,
            permeability,
            max,
            rate,
            elapsed: 0.0,
            on_interval,
            off_interval,
            replenish: true,
        }
    }
}
impl<const N: usize, const KN: usize> Env<N> for PeriodicReplenishConvolve<N, KN> {
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
            self.replenish(dt);
        }
        self.convolve(dt);
    }
}
impl<const N: usize, const KN: usize> ReplenishTrait<N> for PeriodicReplenishConvolve<N, KN> {
    fn rate(&self) -> f32 {
        self.rate
    }
}
impl<const N: usize, const KN: usize> ConvolveTrait<N, KN> for PeriodicReplenishConvolve<N, KN> {
    fn kernel(&self) -> Field<f32, KN> {
        self.kernel
    }

    fn permeability(&self) -> f32 {
        self.permeability
    }
}
impl<const N: usize, const KN: usize> Index<usize> for PeriodicReplenishConvolve<N, KN> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.field[index]
    }
}
impl<const N: usize, const KN: usize> IndexMut<usize> for PeriodicReplenishConvolve<N, KN> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.field[index]
    }
}
// impl<const N: usize, const KN: usize> Env for PeriodicReplenishConvolve<N, KN> {
//     fn get(&self, x: isize, y: isize) -> f32 {
//         self.replenish_convolve.get(x, y)
//     }

//     // fn set(&mut self, x: isize, y: isize, value: f32) {
//     //     self.field.
//     // }

//     fn delta(&mut self, x: isize, y: isize, delta: &mut f32) {
//         self.replenish_convolve.delta(x, y, delta);
//     }

//     fn max(&self) -> f32 {
//         self.replenish_convolve.max()
//     }

//     fn update(&mut self, mut dt: f32) {
//         self.elapsed += dt;

//         let diff = match self.replenish {
//             true => self.elapsed - self.on_interval,
//             false => self.elapsed - self.off_interval,
//         };

//         if diff.is_positive() {
//             self.replenish = !self.replenish;
//             self.elapsed = 0.0;
//             dt = diff;
//         }

//         if self.replenish {
//             self.replenish_convolve.replenish(dt);
//         }
//         self.replenish_convolve.convolve.convolve(dt);
//     }
// }

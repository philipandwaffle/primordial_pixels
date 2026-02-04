use std::ops::Index;

use crate::world::environment::{
    accessor_trait::Env,
    layer::{convolve::Convolve, replenish::Replenish},
};

pub enum LayerType<const N: usize, const KN: usize> {
    Replenish(Replenish<N>),
    Convolve(Convolve<N, KN>),
}
impl<const N: usize, const KN: usize> Index<usize> for LayerType<N, KN> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            LayerType::Replenish(replenish) => &replenish[index],
            LayerType::Convolve(convolve) => &convolve[index],
        }
    }
}
impl<const N: usize, const KN: usize> Env for LayerType<N, KN> {
    fn get(&self, x: isize, y: isize) -> f32 {
        match self {
            LayerType::Replenish(replenish_layer) => replenish_layer.get(x, y),
            LayerType::Convolve(convolve_layer) => convolve_layer.get(x, y),
        }
    }

    // fn set(&mut self, x: isize, y: isize, value: f32) {
    //     match self {
    //         LayerType::Replenish(replenish_layer) => replenish_layer.set(x, y, value),
    //         LayerType::Convolve(convolve_layer) => convolve_layer.set(x, y, value),
    //     }
    // }

    fn delta(&mut self, x: isize, y: isize, delta: &mut f32) {
        match self {
            LayerType::Replenish(replenish_layer) => replenish_layer.delta(x, y, delta),
            LayerType::Convolve(convolve_layer) => convolve_layer.delta(x, y, delta),
        }
    }

    fn max(&self) -> f32 {
        match self {
            LayerType::Replenish(replenish) => replenish.max(),
            LayerType::Convolve(convolve) => convolve.max(),
        }
    }

    fn update(&mut self, dt: f32) {
        match self {
            LayerType::Replenish(replenish_layer) => replenish_layer.update(dt),
            LayerType::Convolve(convolve_layer) => convolve_layer.update(dt),
        }
    }
}

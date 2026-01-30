use crate::world::environment::{
    accessor_trait::EnvAccessor,
    layer::{convolve::Convolve, replenish::Replenish},
};

pub enum LayerType<const N: usize, const KN: usize> {
    Replenish(Replenish<N>),
    Convolve(Convolve<N, KN>),
}
impl<const N: usize, const KN: usize> EnvAccessor for LayerType<N, KN> {
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
}
impl<const N: usize, const KN: usize> LayerType<N, KN> {
    pub fn update(&mut self, dt: f32) {
        match self {
            LayerType::Replenish(replenish_layer) => replenish_layer.replenish(dt),
            LayerType::Convolve(convolve_layer) => convolve_layer.convolve(dt),
        }
    }
}

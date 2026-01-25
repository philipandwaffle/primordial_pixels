use crate::world::environment::layer::{ConvolveLayer, ReplenishLayer};

pub enum LayerType<const N: usize, const KN: usize> {
    Replenish(ReplenishLayer<N>),
    Convolve(ConvolveLayer<N, KN>),
}
impl<const N: usize, const KN: usize> LayerType<N, KN> {
    pub fn update(&mut self, dt: f32) {
        match self {
            LayerType::Replenish(replenish_layer) => replenish_layer.replenish(dt),
            LayerType::Convolve(convolve_layer) => convolve_layer.convolve(dt),
        }
    }
}

use avian2d::parry::utils::hashmap::HashMap;

use crate::world::environment::{layer_key::LayerKey, layer_type::LayerType};

pub struct Environment<const N: usize, const KN: usize> {
    layers: HashMap<LayerKey, LayerType<N, KN>>,
}
impl<const N: usize, const KN: usize> Environment<N, KN> {
    // TODO: move this to a trait?
    pub fn update(&mut self, dt: f32) {
        for l in self.layers.values_mut() {
            l.update(dt);
        }
    }
}

use std::collections::HashMap;

use bevy::{ecs::resource::Resource, math::Vec2};

use crate::world::environment::{
    accessor_trait::EnvAccessor, field::Field, layer::convolve::Convolve, layer_key::LayerKey,
    layer_type::LayerType,
};

#[derive(Resource)]
pub struct Environment<const N: usize, const KN: usize> {
    size: Vec2,
    layers: HashMap<LayerKey, LayerType<N, KN>>,
}
impl<const N: usize, const KN: usize> Environment<N, KN> {
    pub fn new(size: Vec2) -> Self {
        let mut layers = HashMap::<LayerKey, LayerType<N, KN>>::new();
        layers.insert(
            LayerKey::Energy,
            LayerType::Convolve(Convolve::new(
                0.0,
                Field::<KN>::from_array([1.0 / 9.0; KN]),
                5.0,
            )),
        );
        Self { size, layers }
    }
    // TODO: move this to a trait?
    pub fn update(&mut self, dt: f32) {
        for l in self.layers.values_mut() {
            l.update(dt);
        }
    }

    fn world_to_coord(&self, pos: Vec2) -> [isize; 2] {
        let x = (pos.x / self.size.x).round() as isize;
        let y = (pos.y / self.size.y).round() as isize;
        [x, y]
    }

    pub fn get_value(&self, layer: &LayerKey, pos: Vec2) -> f32 {
        let [x, y] = self.world_to_coord(pos);
        self.layers[layer].get(x, y)
    }
    pub fn delta_value(&mut self, layer: &LayerKey, pos: Vec2, delta: &mut f32) {
        let [x, y] = self.world_to_coord(pos);

        if let Some(val) = self.layers.get_mut(layer) {
            val.delta(x, y, delta);
        }
    }
}

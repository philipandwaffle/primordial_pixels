use std::{collections::HashMap, ops::Index};

use bevy::{ecs::resource::Resource, math::Vec2};

use crate::{
    consts::{KN, N},
    world::environment::{
        accessor_trait::Env,
        field::Field,
        layer::{
            convolve::Convolve, layer_key::LayerKey, layer_type::LayerType, replenish::Replenish,
        },
    },
};

#[derive(Resource)]
pub struct Environment<const N: usize, const KN: usize> {
    pub size: Vec2,
    layers: HashMap<LayerKey, LayerType<N, KN>>,
}
impl<const N: usize, const KN: usize> Index<&LayerKey> for Environment<N, KN> {
    type Output = LayerType<N, KN>;

    fn index(&self, index: &LayerKey) -> &Self::Output {
        &self.layers[index]
    }
}
impl<const N: usize, const KN: usize> Environment<N, KN> {
    pub fn new(size: Vec2) -> Self {
        let mut layers = HashMap::<LayerKey, LayerType<N, KN>>::new();
        layers.insert(
            LayerKey::Energy,
            LayerType::Replenish(Replenish::new(0.0, 5.0, 0.25)),
        );
        layers.insert(
            LayerKey::Pheromone(0),
            LayerType::Convolve(Convolve::new(
                0.0,
                Field::<f32, KN>::from_array([1.0 / 9.0; KN]),
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

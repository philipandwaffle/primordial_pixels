use std::{collections::HashMap, ops::Index};

use bevy::{
    ecs::resource::Resource,
    math::{Vec2, vec2},
};

use crate::world::environment::{
    accessor_trait::Env,
    field::Field,
    layer::{
        convolve::Convolve, layer_key::LayerKey, layer_type::LayerType, replenish::Replenish,
        replenish_convolve::ReplenishConvolve,
    },
};

#[derive(Resource)]
pub struct Environment<const N: usize, const KN: usize> {
    pub side_len: f32,
    pub cell_len: f32,
    offset: Vec2,
    layers: HashMap<LayerKey, LayerType<N, KN>>,
}
impl<const N: usize, const KN: usize> Index<&LayerKey> for Environment<N, KN> {
    type Output = LayerType<N, KN>;

    fn index(&self, index: &LayerKey) -> &Self::Output {
        &self.layers[index]
    }
}
impl<const N: usize, const KN: usize> Environment<N, KN> {
    pub fn new(side_len: f32) -> Self {
        let mut layers = HashMap::<LayerKey, LayerType<N, KN>>::new();
        layers.insert(
            LayerKey::Energy,
            LayerType::ReplenishConvolve(ReplenishConvolve::new(
                Convolve::new(0.0, Field::<f32, KN>::from_array([1.0 / 9.0; KN]), 5.0),
                0.05,
            )),
        );
        layers.insert(
            LayerKey::Pheromone(0),
            LayerType::Convolve(Convolve::new(
                0.0,
                Field::<f32, KN>::from_array([0.9 / 9.0; KN]),
                5.0,
            )),
        );
        Self {
            side_len,
            cell_len: side_len / (N as f32).sqrt(),
            offset: vec2(side_len, side_len) * 0.5 - vec2(0.5, 0.5),
            layers,
        }
    }
    // TODO: move this to a trait?
    pub fn update(&mut self, dt: f32) {
        for l in self.layers.values_mut() {
            l.update(dt);
        }
    }

    pub fn world_to_coord(&self, mut pos: Vec2) -> [isize; 2] {
        // println!("pos: {}", pos);
        pos += self.offset;
        // println!("offset_pos: {}", pos);
        let x = (pos.x / self.cell_len).round() as isize;
        let y = (pos.y / self.cell_len).round() as isize;
        // println!("index: [{},{}]", x, y);
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

#[cfg(test)]
mod test {
    use bevy::math::vec2;

    use crate::world::environment::environment::Environment;
    pub fn get_env() -> Environment<9, 9> {
        Environment::new(3.0)
    }

    #[test]
    pub fn pos_x_pos_y() {
        let env = get_env();

        assert_eq!(env.world_to_coord(vec2(1.4, 1.4)), [2, 2]);
        assert_eq!(env.world_to_coord(vec2(0.6, 0.6)), [2, 2]);
    }

    #[test]
    pub fn pos_x_nex_y() {
        let env = get_env();

        assert_eq!(env.world_to_coord(vec2(1.4, -0.6)), [2, 0]);
        assert_eq!(env.world_to_coord(vec2(0.6, -1.4)), [2, 0]);
    }

    #[test]
    pub fn zero_x_zero_y() {
        let env = get_env();

        assert_eq!(env.world_to_coord(vec2(0.4, 0.4)), [1, 1]);
        assert_eq!(env.world_to_coord(vec2(-0.4, -0.4)), [1, 1]);
    }

    #[test]
    pub fn neg_x_pos_y() {
        let env = get_env();

        assert_eq!(env.world_to_coord(vec2(-0.6, 1.4)), [0, 2]);
        assert_eq!(env.world_to_coord(vec2(-1.4, 0.6)), [0, 2]);
    }

    #[test]
    pub fn neg_x_neg_y() {
        let env = get_env();

        assert_eq!(env.world_to_coord(vec2(-0.6, -0.6)), [0, 0]);
        assert_eq!(env.world_to_coord(vec2(-1.4, -1.4)), [0, 0]);
    }
}

use std::ops::Index;

use bevy::{
    asset::{Assets, Handle},
    color::{Color, Hue},
    ecs::{entity::Entity, resource::Resource},
    sprite_render::ColorMaterial,
};

use crate::{
    consts::{N, NUM_COLORS},
    world::environment::{field::Field, layer::layer_key::LayerKey},
};

#[derive(Resource)]
pub struct Display {
    pub cur_layer: LayerKey,
    pub field: Field<Entity, N>,
    pub colors: Vec<Handle<ColorMaterial>>,
}
impl Index<usize> for Display {
    type Output = Entity;

    fn index(&self, index: usize) -> &Self::Output {
        &self.field.space[index]
    }
}
impl Display {
    pub fn new(mats: &mut Assets<ColorMaterial>) -> Self {
        let cur_hue = 230.0;
        let mut color = Color::hsla(cur_hue, 1.0, 0.5, 0.25);
        let delta = (360.0 - cur_hue) / NUM_COLORS as f32;

        let mut colors = Vec::with_capacity(NUM_COLORS);
        colors.push(mats.add(color));
        for _ in 1..NUM_COLORS {
            colors.push(mats.add(color));
            color = color.rotate_hue(delta);
        }

        Self {
            cur_layer: LayerKey::Energy,
            field: Field::from_element(Entity::PLACEHOLDER),
            colors,
        }
    }
}

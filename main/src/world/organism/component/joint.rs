use bevy::ecs::{component::Component, entity::Entity};

use crate::world::organism::node_type::NodeType;

#[derive(Component)]
pub struct Joint {
    pub nodes: Vec<NodeType>,
    pub thruster: Option<Entity>,
    pub spike: Option<Entity>,
    pub eyes: Option<Vec<Entity>>,
}
impl Joint {
    pub fn new(
        nodes: &Vec<NodeType>,
        thruster: Option<Entity>,
        spike: Option<Entity>,
        eyes: Option<Vec<Entity>>,
    ) -> Self {
        Self {
            nodes: nodes.clone(),
            thruster,
            spike,
            eyes,
        }
    }
}

#[derive(Component)]
pub struct Thruster;

#[derive(Component)]
pub struct Spike;

#[derive(Component)]
pub struct Eye {
    rays: Vec<Entity>,
}
impl Eye {
    pub fn new(rays: Vec<Entity>) -> Self {
        return Self { rays };
    }
    pub fn get_ray_ents(&self) -> &Vec<Entity> {
        &self.rays
    }

    pub fn with_rays(&mut self, rays: Vec<Entity>) {
        self.rays = rays;
    }
}

#[derive(Component)]
pub struct EyeRay;

use avian2d::prelude::{Collider, RigidBody};
use bevy::{
    ecs::{
        component::Component,
        entity::{Entity, EntityHashSet},
        query::With,
        system::{Commands, Query},
    },
    log::info,
    math::Vec2,
    transform::components::Transform,
};
use serde::de;

use crate::{
    config::config::Metabolism,
    world::organism::{
        body::Body,
        brain::Brain,
        component::joint::Joint,
        organism::Organism,
        seed::Seed,
        stats::{Stats, VariableStats},
        util_trait::OrganismAccessor,
    },
};

#[derive(Component)]
pub struct OrganismMarker {
    pub organism: Organism,
    cur_energy: f32,
    pub max_energy: f32,
    variable_stats: VariableStats,
    pub joint_ents: Vec<Entity>,
    pub bone_ents: Vec<Entity>,
    pub muscle_ents: Vec<Entity>,
    pub col_ents: EntityHashSet,
}
impl OrganismMarker {
    pub fn new(
        organism: Organism,
        joints: Vec<Entity>,
        bones: Vec<Entity>,
        muscles: Vec<Entity>,
    ) -> Self {
        let max_energy = organism.max_energy();
        let mut col_ents = EntityHashSet::new();
        {
            for e in bones.iter() {
                col_ents.insert(*e);
            }
            for e in muscles.iter() {
                col_ents.insert(*e);
            }
        }

        Self {
            organism,
            cur_energy: max_energy * 0.2,
            max_energy,
            variable_stats: VariableStats::new(),
            joint_ents: joints,
            bone_ents: bones,
            muscle_ents: muscles,
            col_ents,
        }
    }

    // pub fn owns_col(&self, ent: &Entity) -> bool {
    //     self.col_ents.contains(ent)
    // }

    pub fn update_energy(&mut self, delta: f32) {
        self.cur_energy += delta;
        if self.cur_energy > self.max_energy {
            self.cur_energy = self.max_energy;
        }
    }

    pub fn impale(&mut self, delta: f32) -> f32 {
        self.cur_energy -= delta;

        let extracted = if self.cur_energy < 0.0 {
            delta + self.cur_energy
        } else {
            delta
        };

        -extracted
    }

    pub fn get_energy_level(&self) -> f32 {
        self.cur_energy / self.max_energy
    }

    pub fn is_dead(&self) -> bool {
        return self.cur_energy <= 0.0;
    }

    pub fn reproduce(
        &mut self,
        metabolism: &Metabolism,
        joint_query: &Query<&Transform, With<Joint>>,
    ) -> Option<Seed> {
        if self.cur_energy <= self.max_energy * metabolism.reproduce_threshold {
            return None;
        }

        self.cur_energy -= self.max_energy * metabolism.reproduce_cost;
        Some(self.as_seed(self.get_pos(joint_query)))
    }

    pub fn get_pos(&self, joint_query: &Query<&Transform, With<Joint>>) -> Vec2 {
        // let test = self
        //     .joint_ents
        //     .iter()
        //     .map(|j_ent| joint_query.get(*j_ent).unwrap().translation.truncate())
        //     .collect::<Vec<Vec2>>();
        // println!("{:?}", test);

        self.joint_ents
            .iter()
            .map(|j_ent| joint_query.get(*j_ent).unwrap().translation.truncate())
            .sum::<Vec2>()
            / (self.joint_ents.len() as f32)
    }

    pub fn update_variable_stats(&mut self, dt: f32) {
        self.variable_stats.time_alive += dt;
    }

    pub fn get_static_stats<'a>(&'a self) -> &'a Stats {
        return self.organism.get_static_stats();
    }
    pub fn get_variable_stats<'a>(&'a self) -> &'a VariableStats {
        return &self.variable_stats;
    }

    pub fn as_seed(&self, pos: Vec2) -> Seed {
        self.organism.clone().as_seed(pos)
    }

    pub fn despawn(&mut self, commands: &mut Commands) {
        Self::despawn_ent_vec(commands, &mut self.muscle_ents);
        Self::despawn_ent_vec(commands, &mut self.bone_ents);
        Self::despawn_ent_vec(commands, &mut self.joint_ents);
    }

    fn despawn_ent_vec(commands: &mut Commands, ents: &mut Vec<Entity>) {
        while !ents.is_empty() {
            commands
                .entity(ents.pop().unwrap())
                .remove::<RigidBody>()
                .despawn();
        }
    }
}
impl OrganismAccessor for OrganismMarker {
    fn get_mut_organism<'a>(&'a mut self) -> &'a mut Organism {
        return &mut self.organism;
    }

    fn get_mut_body<'a>(&'a mut self) -> &'a mut Body {
        return &mut self.organism.body;
    }

    fn get_mut_brain<'a>(&'a mut self) -> Option<&'a mut Brain> {
        return self.organism.brain.as_mut();
    }

    fn get_organism<'a>(&'a self) -> &'a Organism {
        return &self.organism;
    }
    fn get_body<'a>(&'a self) -> &'a Body {
        return &self.organism.body;
    }

    fn get_brain<'a>(&'a self) -> &'a Option<Brain> {
        return &self.organism.brain;
    }
}

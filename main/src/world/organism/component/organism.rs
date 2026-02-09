use bevy::{
    ecs::{component::Component, entity::Entity, system::Commands},
    math::Vec2,
};

use crate::world::organism::{
    body::Body,
    brain::Brain,
    organism::Organism,
    seed::Seed,
    stats::{StaticStats, VariableStats},
    util_trait::OrganismAccessor,
};

#[derive(Component)]
pub struct OrganismEntity {
    pub organism: Organism,
    pub cur_energy: f32,
    max_energy: f32,
    variable_stats: VariableStats,
    pub joint_ents: Vec<Entity>,
    pub bone_ents: Vec<Entity>,
    pub muscle_ents: Vec<Entity>,
}
impl OrganismEntity {
    pub fn new(
        organism: Organism,
        joints: Vec<Entity>,
        bones: Vec<Entity>,
        muscles: Vec<Entity>,
    ) -> Self {
        let max_energy = organism.max_energy();
        Self {
            organism,
            cur_energy: max_energy * 0.5,
            max_energy,
            joint_ents: joints,
            bone_ents: bones,
            muscle_ents: muscles,
            variable_stats: VariableStats::new(),
        }
    }

    pub fn update_variable_stats(&mut self, dt: f32) {
        self.variable_stats.time_alive += dt;
    }

    pub fn get_static_stats<'a>(&'a self) -> &'a StaticStats {
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
            commands.entity(ents.pop().unwrap()).despawn();
        }
    }
}
impl OrganismAccessor for OrganismEntity {
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

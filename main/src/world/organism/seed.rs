use avian2d::prelude::{
    Collider, DistanceJoint, LinearDamping, LockedAxes, RevoluteJoint, RigidBody,
};
use bevy::{
    ecs::{entity::Entity, system::Commands},
    math::{Quat, Vec2, vec2, vec3},
    transform::components::Transform,
};
use my_derive::ConfigTag;
use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};

use crate::{
    assets::handles::{Handles, MatKey, MeshKey},
    config::{
        config::{Metabolism, Mutation as MutationConfig},
        config_tag::ConfigTag,
        plugin::load_config,
    },
    consts::{
        BONE_WIDTH, BONE_Z, JOINT_RADIUS, JOINT_Z, MUSCLE_COMPLIANCE, MUSCLE_WIDTH, MUSCLE_Z,
        PHYS_LOCK_DUR, PHYS_LOCK_FINAL_DAMP, PHYS_LOCK_START_DAMP,
    },
    physics_lock::PhysicsLockBundle,
    world::organism::{
        body::Body,
        brain::Brain,
        component::{
            bone::Bone, joint::Joint as JointComp, muscle::Muscle, organism::OrganismMarker,
        },
        joint::Joint,
        message::SpawnOrganismMsg,
        mutation::{
            brain::Brain as BrainMut,
            mutation::{Mut, Mutable, Mutation as OrgMut},
        },
        node::{energy::Energy, thruster::Thruster},
        node_type::NodeType,
        organism::Organism,
        util_trait::OrganismAccessor,
    },
};

#[derive(Clone, ConfigTag, Debug, Serialize, Deserialize)]
pub struct Seed {
    pos: Vec2,
    organism: Organism,
}
impl Default for Seed {
    fn default() -> Self {
        let cfg = load_config();
        Self {
            pos: Default::default(),
            organism: Organism::new(
                // Some(Brain::new(vec![1, 5, 5, 2])),
                None,
                Body::new(
                    vec![Joint::new(
                        vec2(0.0, 0.0),
                        vec![
                            NodeType::Energy(Energy::new()),
                            // NodeType::Thruster(Thruster::new(0.0)),
                        ],
                    )],
                    vec![],
                    vec![],
                ),
                cfg.organism.metabolism,
            ),
        }
    }
}
impl Mutable for Seed {
    fn mutate(&mut self, mutation: &OrgMut) -> bool {
        self.organism.mutate(mutation)
    }
}
impl OrganismAccessor for Seed {
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
impl Seed {
    pub fn new(pos: Vec2, organism: Organism) -> Self {
        Self { pos, organism }
    }

    pub fn multi_mutate(
        &mut self,
        rng: &mut ThreadRng,
        metabolism: &Metabolism,
        mutation_config: &MutationConfig,
        num_muts: usize,
    ) {
        for _ in 0..num_muts {
            if let Some(m) = OrgMut::rand(rng, &mutation_config, self.get_organism()) {
                self.mutate(&m);
            }
        }
        self.mutate(&OrgMut::Brain(
            BrainMut::rand(rng, mutation_config, self.get_organism()).unwrap(),
        ));
        self.update_metabolic_cost(&metabolism);
    }

    pub fn update_metabolic_cost(&mut self, metabolism: &Metabolism) {
        self.organism.update_metabolic_cost(metabolism)
    }

    pub fn centre(&mut self) {
        self.organism.centre();
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }
}
impl Into<SpawnOrganismMsg> for Seed {
    fn into(self) -> SpawnOrganismMsg {
        SpawnOrganismMsg::new(self.pos, self.organism)
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use bevy::{math::vec2, ui_render::NodeType};

    use crate::{
        config::{config::Config as CFG, config_tag::Config, plugin::load_config},
        consts::CONFIG_PATH,
        world::organism::{body::Body, brain::Brain, joint::Joint, organism::Organism, seed::Seed},
    };

    #[test]
    fn seed_save_load() {
        let config = CFG::load_cfg(Path::new(&format!("../{CONFIG_PATH}")));
        let seed = Seed::new(
            vec2(0.0, 0.0),
            Organism::new(
                Some(Brain::default()),
                Body::new(vec![Joint::new(vec2(0.0, 0.0), vec![])], vec![], vec![]),
                config.organism.metabolism,
            ),
        );
        let path = Path::new("tmp/organism.json");
        seed.save_cfg(&path);
        // fs::remove_file(path);
    }
}

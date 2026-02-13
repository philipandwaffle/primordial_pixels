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

    pub fn spawn(&self, commands: &mut Commands, h: &Handles) -> Entity {
        let offset = self.pos;
        let o = &self.organism;

        let mut joint_ents = Vec::with_capacity(o.body.joints.len());
        let mut joint_positions = Vec::with_capacity(o.body.joints.len());

        let mut bone_ents = Vec::with_capacity(o.body.bones.len());
        let mut bone_positions = Vec::with_capacity(o.body.bones.len());

        let mut muscle_ents = Vec::with_capacity(o.body.muscles.len());

        for j in o.body.joints.iter() {
            let pos = offset + j.pos;
            let j_ent = Self::spawn_joint(pos, &j.nodes, commands, h);

            joint_ents.push(j_ent);
            joint_positions.push(pos);
        }

        for b in o.body.bones.iter() {
            let (b_ent, pos) =
                // Self::spawn_bone(offset, b, &joint_ents, &joint_positions, commands, h);
                Self::spawn_bone( b, &joint_ents, &joint_positions, commands, h);

            bone_ents.push(b_ent);
            bone_positions.push(pos);
        }

        for m in o.body.muscles.iter() {
            let m_ent = Self::spawn_muscle(m, &bone_ents, &bone_positions, commands, h);
            muscle_ents.push(m_ent);
        }

        commands
            .spawn((
                OrganismMarker::new(
                    o.clone(),
                    joint_ents.clone(),
                    bone_ents.clone(),
                    muscle_ents.clone(),
                ),
                Transform::default(),
            ))
            .add_children(joint_ents.as_slice())
            .add_children(bone_ents.as_slice())
            .add_children(muscle_ents.as_slice())
            .id()
    }

    pub fn spawn_joint(pos: Vec2, nodes: &Vec<NodeType>, c: &mut Commands, h: &Handles) -> Entity {
        c.spawn((
            LockedAxes::ROTATION_LOCKED,
            PhysicsLockBundle::new(PHYS_LOCK_DUR, PHYS_LOCK_START_DAMP, PHYS_LOCK_FINAL_DAMP),
            JointComp::new(nodes),
            RigidBody::Dynamic,
            Transform::default()
                .with_translation(pos.extend(JOINT_Z))
                .with_scale(vec3(JOINT_RADIUS, JOINT_RADIUS, 1.0)),
            Collider::circle(1.0),
            h.get_mesh2d(&MeshKey::Circle),
            h.get_mat2d(&MatKey::Green),
        ))
        .id()
    }

    fn spawn_bone(
        // offset: Vec2,
        bone: &[usize; 2],
        j_ents: &Vec<Entity>,
        j_pos: &Vec<Vec2>,
        c: &mut Commands,
        h: &Handles,
    ) -> (Entity, Vec2) {
        let [a, b] = *bone;
        let pos_a = j_pos[a];
        let pos_b = j_pos[b];

        let mid = pos_a.midpoint(pos_b);
        let dir = pos_b - pos_a;
        let z_rot = dir.to_angle();
        let length = dir.length();
        let bone_length = length - (JOINT_RADIUS * 2.1);

        let bone_ent = c
            .spawn((
                Bone,
                RigidBody::Dynamic,
                Transform::default()
                    .with_translation((mid).extend(BONE_Z))
                    .with_rotation(Quat::from_rotation_z(z_rot))
                    .with_scale(vec3(bone_length, BONE_WIDTH, 1.0)),
                Collider::rectangle(1.0, 1.0),
                h.get_mesh2d(&MeshKey::Rectangle),
                h.get_mat2d(&MatKey::White),
            ))
            .id();

        // c.entity(bone_ent)
        //     .with_child(RevoluteJoint::new(bone_ent, j_ents[a]).with_anchor(offset + pos_a));
        // c.entity(bone_ent)
        //     .with_child(RevoluteJoint::new(bone_ent, j_ents[b]).with_anchor(offset + pos_b));
        c.entity(bone_ent)
            .with_child(RevoluteJoint::new(bone_ent, j_ents[a]).with_anchor(pos_a));
        c.entity(bone_ent)
            .with_child(RevoluteJoint::new(bone_ent, j_ents[b]).with_anchor(pos_b));

        (bone_ent, mid)
    }

    fn spawn_muscle(
        muscle: &[usize; 2],
        b_ents: &Vec<Entity>,
        b_pos: &Vec<Vec2>,
        c: &mut Commands,
        h: &Handles,
    ) -> Entity {
        let [a, b] = *muscle;
        let pos_a = b_pos[a];
        let pos_b = b_pos[b];

        let mid = pos_a.midpoint(pos_b);
        let dir = pos_b - pos_a;
        let z_rot = dir.to_angle();
        let length = dir.length();

        c.spawn((
            Muscle::new([b_ents[a], b_ents[b]], length),
            DistanceJoint::new(b_ents[a], b_ents[b])
                // .with_local_anchor1(mid - pos_a)
                // .with_local_anchor2(mid - pos_b)
                .with_limits(length, length)
                .with_compliance(MUSCLE_COMPLIANCE),
            Transform::default()
                .with_translation((mid).extend(MUSCLE_Z))
                .with_rotation(Quat::from_rotation_z(z_rot))
                .with_scale(vec3(length, MUSCLE_WIDTH, 1.0)),
            h.get_mesh2d(&MeshKey::Rectangle),
            h.get_mat2d(&MatKey::Red),
        ))
        .id()
    }
}

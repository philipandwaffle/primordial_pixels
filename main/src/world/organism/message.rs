use avian2d::prelude::{
    Collider, DistanceJoint, LinearDamping, LockedAxes, RevoluteJoint, RigidBody,
};
use bevy::{
    ecs::{entity::Entity, message::Message, system::Commands},
    math::{Quat, Vec2, Vec3, VectorSpace, vec2, vec3},
    transform::components::Transform,
};

use rand::rngs::ThreadRng;

use crate::{
    assets::handles::{Handles, MatKey, MeshKey},
    consts::{
        BONE_WIDTH, BONE_Z, EGG_Z, JOINT_RADIUS, JOINT_Z, LINEAR_DAMPING, MIN_EGG_RADIUS,
        MUSCLE_COMPLIANCE, MUSCLE_WIDTH, MUSCLE_Z, THRUSTER_BASE_LENGTH, THRUSTER_WIDTH,
        THRUSTER_Z,
    },
    util::function::rand_vec2,
    world::organism::{
        component::{
            bone::Bone,
            egg::Egg,
            joint::{Joint as JointComp, Thruster as ThrusterComp},
            muscle::Muscle,
            organism::OrganismMarker,
        },
        node_type::NodeType,
        organism::Organism,
    },
};

#[derive(Message)]
pub struct DespawnOrganismMsg {
    pub(crate) entity: Entity,
}
impl DespawnOrganismMsg {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

#[derive(Message)]
pub struct SpawnEggMsg {
    pos: Vec2,
    incubation_period: f32,
    organism: Organism,
}
impl SpawnEggMsg {
    pub fn new(pos: Vec2, incubation_period: f32, organism: Organism) -> Self {
        Self {
            pos,
            incubation_period,
            organism,
        }
    }

    pub fn spawn(&self, commands: &mut Commands, h: &Handles, rng: &mut ThreadRng) -> Entity {
        commands
            .spawn((
                LockedAxes::ROTATION_LOCKED,
                LinearDamping(LINEAR_DAMPING),
                Egg::new(self.incubation_period, self.organism.clone()),
                Transform::default()
                    .with_translation((self.pos + rand_vec2(rng, JOINT_RADIUS * 0.5)).extend(EGG_Z))
                    .with_scale(vec3(MIN_EGG_RADIUS, MIN_EGG_RADIUS, MIN_EGG_RADIUS)),
                RigidBody::Dynamic,
                Collider::circle(1.0),
                // Mass(1.0),
                h.get_mesh2d(&MeshKey::Circle),
                h.get_mat2d(&MatKey::White),
            ))
            .id()
    }
}

#[derive(Message)]
pub struct SpawnOrganismMsg {
    pos: Vec2,
    organism: Organism,
}
impl SpawnOrganismMsg {
    pub fn new(pos: Vec2, organism: Organism) -> Self {
        Self { pos, organism }
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
        let mut has_thruster = false;
        for n in nodes {
            match n {
                NodeType::Thruster(_) => has_thruster = true,
                _ => {}
            }
        }

        let thruster_ent = match has_thruster {
            true => Some(Self::spawn_thruster(c, h)),
            false => None,
        };

        let joint_ent = c
            .spawn((
                LockedAxes::ROTATION_LOCKED,
                LinearDamping(LINEAR_DAMPING),
                // PhysicsLockBundle::new(PHYS_LOCK_DUR, PHYS_LOCK_START_DAMP, PHYS_LOCK_FINAL_DAMP),
                JointComp::new(nodes, thruster_ent),
                RigidBody::Dynamic,
                Transform::default()
                    .with_translation(pos.extend(JOINT_Z))
                    .with_scale(vec3(JOINT_RADIUS, JOINT_RADIUS, 1.0)),
                Collider::circle(1.0),
                h.get_mesh2d(&MeshKey::Circle),
                h.get_mat2d(&MatKey::Green),
            ))
            .id();

        if let Some(thruster_ent) = thruster_ent {
            c.entity(joint_ent).add_child(thruster_ent);
        }

        joint_ent
    }
    pub fn spawn_thruster(c: &mut Commands, h: &Handles) -> Entity {
        c.spawn((
            ThrusterComp,
            Transform::default()
                .with_translation((vec2(0.0, THRUSTER_BASE_LENGTH * 0.5)).extend(THRUSTER_Z))
                .with_scale(vec3(THRUSTER_WIDTH, THRUSTER_BASE_LENGTH, 1.0)),
            h.get_mesh2d(&MeshKey::Triangle),
            h.get_mat2d(&MatKey::Orange),
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

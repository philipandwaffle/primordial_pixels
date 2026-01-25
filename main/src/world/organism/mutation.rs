use bevy::math::Vec2;
use rand::{Rng, rngs::ThreadRng, seq::SliceRandom};

use crate::{
    config::config::Organism as OrganismConfig,
    util::{rand_normal_vec2, shuffled_indexes},
    world::organism::{component::Muscle, node_type::NodeType, organism::Organism},
};

#[derive(Debug)]
pub enum Mutation {
    Body(Body),
    Brain(Brain),
}
impl Mut for Mutation {
    fn gen_mutation(rng: &mut ThreadRng, oc: &OrganismConfig, o: &Organism) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(body_mutation) = Body::gen_mutation(rng, oc, o) {
            return Some(Mutation::Body(body_mutation));
        }
        None
    }

    fn get_cascading(
        &self,
        rng: &mut ThreadRng,
        oc: &OrganismConfig,
        o: &Organism,
    ) -> Option<Vec<Mutation>> {
        match self {
            Mutation::Body(body) => body.get_cascading(rng, oc, o),
            Mutation::Brain(brain) => brain.get_cascading(rng, oc, o),
        }
    }
}

pub trait Mutable {
    fn mutate(&mut self, mutation: Mutation) -> bool;
}

pub trait Mut {
    fn gen_mutation(rng: &mut ThreadRng, oc: &OrganismConfig, o: &Organism) -> Option<Self>
    where
        Self: Sized;

    fn get_cascading(
        &self,
        rng: &mut ThreadRng,
        oc: &OrganismConfig,
        o: &Organism,
    ) -> Option<Vec<Mutation>> {
        return None;
    }
}

#[derive(Debug)]
pub enum Body {
    AddNode { joint: usize, node_type: NodeType },
    AddJoint { pos: Vec2 },
    AddBone { bone: [usize; 2] },
    AddMuscle { muscle: [usize; 2] },
    RemoveNode { joint: usize, node: usize },
    RemoveJoint { joint: usize },
    RemoveBone { bone: usize },
    RemoveMuscle { muscle: usize },
}
impl Body {
    fn gen_edge(
        rng: &mut ThreadRng,
        edges: &Vec<[usize; 2]>,
        num_vertexes: usize,
    ) -> Option<[usize; 2]> {
        if num_vertexes < 2 {
            return None;
        }

        let mut indexes = (0..num_vertexes).collect::<Vec<usize>>();
        indexes.shuffle(rng);
        let a = indexes.pop().unwrap();
        let b = indexes.pop().unwrap();
        let edge = if a > b { [b, a] } else { [a, b] };

        if edges.contains(&edge) {
            return None;
        }
        Some(edge)
    }
}
impl Mut for Body {
    fn gen_mutation(rng: &mut ThreadRng, oc: &OrganismConfig, o: &Organism) -> Option<Self> {
        match rng.gen_range(0..=7) {
            // AddNode
            0 => Some(Body::AddNode {
                joint: rng.gen_range(0..o.body.joints.len()),
                node_type: NodeType::gen_mutation(rng, oc, o)?,
            }),
            // AddJoint
            1 => Some(Body::AddJoint {
                pos: rand_normal_vec2(rng) * rng.gen_range(0.0..10.0),
            }),
            // AddBone
            2 => {
                if let Some(bone) = Self::gen_edge(rng, &o.body.bones, o.body.joints.len()) {
                    return Some(Body::AddBone { bone });
                } else {
                    return None;
                }
            }
            // AddMuscle
            3 => {
                if let Some(muscle) = Self::gen_edge(rng, &o.body.muscles, o.body.bones.len()) {
                    return Some(Body::AddMuscle { muscle });
                } else {
                    return None;
                }
            }
            // RemoveNode
            4 => {
                for i in shuffled_indexes(rng, o.body.joints.len()) {
                    let j = &o.body.joints[i];
                    if !j.nodes.is_empty() {
                        return Some(Body::RemoveNode {
                            joint: i,
                            node: rng.gen_range(0..j.nodes.len()),
                        });
                    }
                }
                None
            }
            // RemoveJoint
            5 => {
                let mut bone_joints = o.body.bones.iter().flat_map(|b| *b).collect::<Vec<usize>>();
                bone_joints.sort();
                for j in shuffled_indexes(rng, o.body.joints.len()) {
                    let edges = bone_joints
                        .iter()
                        .filter(|bone_joint| **bone_joint == j)
                        .collect::<Vec<&usize>>()
                        .len();

                    if edges == 1 {
                        return Some(Body::RemoveJoint { joint: j });
                    }
                }
                None
            }
            // RemoveBone
            6 => {
                let mut muscled_bones = o
                    .body
                    .muscles
                    .iter()
                    .flat_map(|m| *m)
                    .collect::<Vec<usize>>();
                muscled_bones.sort();
                muscled_bones.dedup();

                let bone_joints = o.body.bones.iter().flat_map(|b| *b).collect::<Vec<usize>>();

                let mut bones = o.body.bones.clone();
                bones.shuffle(rng);

                let mut removable_bones = o
                    .body
                    .bones
                    .iter()
                    .enumerate()
                    .filter_map(|(i, bone)| {
                        // if the bone has no attached muscles
                        if !muscled_bones.contains(&i) {
                            // if each side of the bone already has a connection
                            if bone.iter().all(|bone_joint| {
                                bone_joints
                                    .iter()
                                    .filter(|j| *j == bone_joint)
                                    .collect::<Vec<&usize>>()
                                    .len()
                                    > 2
                            }) {
                                return Some(i);
                            }
                        }
                        None
                    })
                    .collect::<Vec<usize>>();

                if removable_bones.is_empty() {
                    return None;
                }

                removable_bones.shuffle(rng);
                Some(Body::RemoveBone {
                    bone: removable_bones.pop().unwrap(),
                })
            }
            // RemoveMuscle
            _ => {
                if !o.body.muscles.is_empty() {
                    return Some(Body::RemoveMuscle {
                        muscle: rng.gen_range(0..o.body.muscles.len()),
                    });
                }
                None
            }
        }
    }

    fn get_cascading(
        &self,
        rng: &mut ThreadRng,
        oc: &OrganismConfig,
        o: &Organism,
    ) -> Option<Vec<Mutation>> {
        match self {
            Body::AddNode { joint, node_type } => node_type.get_cascading(rng, oc, o),
            // match node_type {
            //     NodeType::Energy(energy) => None,
            //     NodeType::PheromoneWrite(pheromone) => todo!(),
            //     NodeType::Thruster(thruster) => todo!(),
            // },
            Body::AddJoint { pos: _ } => {
                let num_joints = o.body.joints.len();
                Some(vec![Mutation::Body(Body::AddBone {
                    bone: [rng.gen_range(0..num_joints), num_joints],
                })])
            }
            Body::AddBone { bone: _ } => None,
            Body::AddMuscle { muscle } => vec![Mutation::Brain(Brain::AddInput { index: o })],
            Body::RemoveNode { joint, node } => todo!(),
            Body::RemoveJoint { joint } => todo!(),
            Body::RemoveBone { bone } => None,
            Body::RemoveMuscle { muscle } => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum Brain {
    AddInput { index: usize },
    RemoveInput { index: usize },
    AddOutput { index: usize },
    RemoveOutput { index: usize },
    Learn { learn_rate: f32, learn_factor: f32 },
}
impl Mut for Brain {
    fn gen_mutation(_: &mut ThreadRng, oc: &OrganismConfig, o: &Organism) -> Option<Self> {
        Some(Brain::Learn {
            learn_rate: oc.learn_rate,
            learn_factor: oc.learn_factor,
        })
    }
}

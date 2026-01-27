use std::collections::HashMap;

use bevy::{core_pipeline::prepass::node, math::Vec2};
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
    fn rand(rng: &mut ThreadRng, oc: &OrganismConfig, o: &Organism) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(body_mutation) = Body::rand(rng, oc, o) {
            return Some(Mutation::Body(body_mutation));
        }
        None
    }

    // fn get_cascading(
    //     &self,
    //     rng: &mut ThreadRng,
    //     oc: &OrganismConfig,
    //     o: &Organism,
    // ) -> Option<Vec<Mutation>> {
    //     match self {
    //         Mutation::Body(body) => body.get_cascading(rng, oc, o),
    //         Mutation::Brain(brain) => brain.get_cascading(rng, oc, o),
    //     }
    // }
}

pub trait Mutable {
    fn mutate(&mut self, mutation: &Mutation) -> bool;
}

pub trait Mut {
    fn rand(rng: &mut ThreadRng, oc: &OrganismConfig, o: &Organism) -> Option<Self>
    where
        Self: Sized;

    // fn get_cascading(
    //     &self,
    //     rng: &mut ThreadRng,
    //     oc: &OrganismConfig,
    //     o: &Organism,
    // ) -> Option<Vec<Mutation>> {
    //     return None;
    // }
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
    fn rand(rng: &mut ThreadRng, oc: &OrganismConfig, o: &Organism) -> Option<Self> {
        match rng.random_range(0..=7) {
            // AddNode
            0 => Some(Body::AddNode {
                joint: rng.random_range(0..o.body.joints.len()),
                node_type: NodeType::rand(rng, oc, o)?,
            }),
            // AddJoint
            1 => Some(Body::AddJoint {
                pos: rand_normal_vec2(rng) * rng.random_range(0.0..10.0),
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
                            node: rng.random_range(0..j.nodes.len()),
                        });
                    }
                }
                None
            }
            // RemoveJoint
            5 => {
                let muscled_bone_indexes = o
                    .body
                    .muscles
                    .iter()
                    .flat_map(|m| *m)
                    .collect::<Vec<usize>>();

                let mut joint_edge_count = HashMap::<usize, usize>::new();
                for [a, b] in o.body.bones.iter().filter(|b| {
                    !(muscled_bone_indexes.contains(&b[0]) || muscled_bone_indexes.contains(&b[1]))
                }) {
                    *joint_edge_count.entry(*a).or_insert(1) += 1;
                    *joint_edge_count.entry(*b).or_insert(1) += 1;
                }

                // Get eligible nodes
                let eligible_nodes = joint_edge_count
                    .iter()
                    .filter(|(_, v)| **v == 1)
                    .map(|(k, _)| *k)
                    .collect::<Vec<usize>>();
                if eligible_nodes.is_empty() {
                    return None;
                }

                let joint = eligible_nodes[rng.random_range(0..eligible_nodes.len())];
                return Some(Body::RemoveJoint { joint });

                // let mut bone_joints = o.body.bones.iter().flat_map(|b| *b).collect::<Vec<usize>>();
                // bone_joints.sort();
                // for j in shuffled_indexes(rng, o.body.joints.len()) {
                //     let edges = bone_joints
                //         .iter()
                //         .filter(|bone_joint| **bone_joint == j)
                //         .collect::<Vec<&usize>>()
                //         .len();

                //     if edges == 1 {
                //         return Some(Body::RemoveJoint { joint: j });
                //     }
                // }
                // None
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
                        muscle: rng.random_range(0..o.body.muscles.len()),
                    });
                }
                None
            }
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
    fn rand(_: &mut ThreadRng, oc: &OrganismConfig, o: &Organism) -> Option<Self> {
        Some(Brain::Learn {
            learn_rate: oc.learn_rate,
            learn_factor: oc.learn_factor,
        })
    }
}
impl Brain {
    pub fn from_in_out(
        [out_con_offset, in_prod_offset]: [usize; 2],
        [out_con, in_prod]: [usize; 2],
        add: bool,
    ) -> Vec<Mutation> {
        let mut res: Vec<_> = vec![];
        for i in 0..out_con {
            let i = out_con_offset + i;

            if add {
                res.push(Mutation::Brain(Brain::AddInput { index: i }));
            } else {
                res.push(Mutation::Brain(Brain::RemoveInput { index: i - 1 }));
            }
        }

        // remove output neurones for node
        for i in 0..in_prod {
            let i = in_prod_offset + i;

            if add {
                res.push(Mutation::Brain(Brain::AddOutput { index: i }));
            } else {
                res.push(Mutation::Brain(Brain::RemoveOutput { index: i - 1 }));
            }
        }
        res
    }
}

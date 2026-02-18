use std::collections::HashMap;

use bevy::math::Vec2;
use rand::{Rng, rngs::ThreadRng, seq::SliceRandom};

use crate::{
    config::config::Mutation as MutationConfig,
    consts::{JOINT_RADIUS, MAX_BONE_LEN, MIN_BONE_LEN},
    util::function::{rand_normal_vec2, rand_vec2, shuffled_indexes},
    world::organism::{
        distribution::Distribution, mutation::mutation::Mut, node_type::NodeType,
        organism::Organism,
    },
};

#[derive(Debug)]
pub enum Body {
    AlterNode {
        joint: usize,
        node: usize,
        node_type: NodeType,
    },
    AddNode {
        joint: usize,
        node_type: NodeType,
    },
    AddJoint {
        joint: usize,
        pos: Vec2,
    },
    MoveJoint {
        joint: usize,
        pos: Vec2,
    },
    AddBone {
        bone: [usize; 2],
    },
    AddMuscle {
        muscle: [usize; 2],
    },
    RemoveNode {
        joint: usize,
        node: usize,
    },
    RemoveJoint {
        joint: usize,
    },
    RemoveBone {
        bone: usize,
    },
    RemoveMuscle {
        muscle: usize,
    },
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
    fn rand(rng: &mut ThreadRng, mutation_config: &MutationConfig, o: &Organism) -> Option<Self> {
        let i = mutation_config.distribution.get_index(rng);
        match i {
            // AlterNode
            0 => {
                let mut alterable_nodes = o
                    .body
                    .joints
                    .iter()
                    .enumerate()
                    .map(|(i, j)| {
                        let mut nodes = j
                            .nodes
                            .iter()
                            .enumerate()
                            .filter(|(_, node)| node.can_alter())
                            .map(|(node, _)| node)
                            .collect::<Vec<usize>>();
                        nodes.shuffle(rng);
                        (i, nodes)
                    })
                    .filter(|(_, nodes)| !nodes.is_empty())
                    .collect::<Vec<(usize, Vec<usize>)>>();
                alterable_nodes.shuffle(rng);

                if alterable_nodes.is_empty() {
                    return None;
                }

                let (joint, nodes) = &alterable_nodes[0];
                let node = nodes[0];

                let node_type = match o.body.joints[*joint].nodes[node] {
                    NodeType::Read(mut read) => {
                        read.read_offset += rand_vec2(rng, JOINT_RADIUS);
                        NodeType::Read(read)
                    }
                    _ => panic!(),
                };
                Some(Body::AlterNode {
                    joint: *joint,
                    node,
                    node_type,
                })
            }
            // AddNode
            1 => {
                let joint = rng.random_range(0..o.body.joints.len());
                let node_type = NodeType::rand(rng, mutation_config, o)?;
                if o.body.joints[joint].nodes.contains(&node_type) {
                    return None;
                }
                Some(Body::AddNode {
                    joint: joint,
                    node_type,
                })
            }
            // AddJoint
            2 => {
                let joint = rng.random_range(0..o.body.joints.len());
                let pos = o.body.joints[joint].pos
                    + rand_normal_vec2(rng) * rng.random_range(MIN_BONE_LEN..MAX_BONE_LEN);

                if o.body
                    .joints
                    .iter()
                    .any(|j| j.pos.distance(pos) < MIN_BONE_LEN)
                {
                    return None;
                } else {
                    return Some(Body::AddJoint { joint, pos });
                }
            }
            // MoveJoint
            3 => {
                let joint = rng.random_range(0..o.body.joints.len());
                let pos = rand_normal_vec2(rng) * rng.random_range(0.0..1.0);

                if o.body
                    .bones
                    .iter()
                    .filter(|bone| bone.contains(&joint))
                    .map(|[a, b]| o.body.joints[*a].pos.distance(o.body.joints[*b].pos))
                    .any(|len| len < MIN_BONE_LEN || len > MAX_BONE_LEN)
                {
                    return None;
                } else {
                    return Some(Body::MoveJoint { joint, pos });
                }
            }
            // AddBone
            4 => {
                if let Some(bone) = Self::gen_edge(rng, &o.body.bones, o.body.joints.len()) {
                    return Some(Body::AddBone { bone });
                } else {
                    return None;
                }
            }
            // AddMuscle
            5 => {
                if let Some(muscle) = Self::gen_edge(rng, &o.body.muscles, o.body.bones.len()) {
                    return Some(Body::AddMuscle { muscle });
                } else {
                    return None;
                }
            }
            // RemoveNode
            6 => {
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
            7 => {
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
            }
            // RemoveBone
            8 => {
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

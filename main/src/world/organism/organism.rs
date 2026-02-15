use bevy::{log::info, math::Vec2};
use serde::{Deserialize, Serialize};

use crate::{
    config::config::Metabolism,
    consts::{
        BASE_INPUT, BASE_OUTPUT, JOINT_MAX_ENERGY, JOINT_RADIUS, MUSCLE_IN_PRODUCE,
        MUSCLE_OUT_CONSUME,
    },
    world::organism::{
        body::Body,
        brain::Brain,
        joint::Joint,
        mutation::{
            body::Body as BodyMut,
            brain::Brain as BrainMut,
            mutation::{Mutable, Mutation},
        },
        out_in::OutputConsumedInputProduced,
        seed::Seed,
        stats::StaticStats,
        transput::Transput,
        util_trait::OrganismAccessor,
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Organism {
    pub brain: Option<Brain>,
    pub body: Body,
    static_stats: StaticStats,
    pub metabolic_cost: f32,
}
impl Mutable for Organism {
    fn mutate(&mut self, mutation: &Mutation) -> bool {
        match mutation {
            Mutation::Body(body) => match body {
                BodyMut::AddNode { joint, node_type } => {
                    let out_in_offset = self.get_organism().joint_out_in(*joint).into();
                    let out_in = node_type.out_in().into();
                    info!("adding node: {:?}", node_type);

                    for m in BrainMut::from_in_out(out_in_offset, out_in, true) {
                        self.mutate(&m);
                    }

                    // Add node
                    self.body.joints[*joint].nodes.push(*node_type);
                    return true;
                }
                BodyMut::AddJoint { joint, pos } => {
                    // Add shortest bone
                    self.mutate(&Mutation::Body(BodyMut::AddBone {
                        bone: [*joint, self.body.joints.len()],
                    }));

                    // Add joint
                    self.body.joints.push(Joint::new(*pos, vec![]));
                    return true;
                }
                BodyMut::MoveJoint { joint, pos } => {
                    self.body.joints[*joint].pos += pos;
                    return true;
                }
                BodyMut::AddBone { bone } => {
                    self.body.bones.push(*bone);
                    return true;
                }
                BodyMut::AddMuscle { muscle } => {
                    let out_in_offset = self.get_organism().muscle_out_in().into();
                    let out_in = [MUSCLE_OUT_CONSUME, MUSCLE_IN_PRODUCE];

                    for m in BrainMut::from_in_out(out_in_offset, out_in, true) {
                        self.mutate(&m);
                    }

                    self.body.muscles.push(*muscle);
                    return true;
                }
                BodyMut::RemoveNode { joint, node } => {
                    let out_in_offset = self.get_organism().joint_node_out_in(*joint, *node).into();
                    let out_in = self.body.joints[*joint].nodes[*node].out_in().into();

                    for m in BrainMut::from_in_out(out_in_offset, out_in, false) {
                        self.mutate(&m);
                    }

                    // remove node
                    self.body.joints[*joint].nodes.remove(*node);
                    return true;
                }
                BodyMut::RemoveJoint { joint } => {
                    self.body.joints.remove(*joint);

                    Self::shift_edges(&mut self.body.bones, joint);
                    return true;
                }
                BodyMut::RemoveBone { bone } => {
                    self.body.bones.remove(*bone);

                    Self::shift_edges(&mut self.body.muscles, bone);
                    return true;
                }
                BodyMut::RemoveMuscle { muscle } => {
                    let out_in_offset = self.get_organism().muscle_out_in().into();
                    let out_in = [MUSCLE_OUT_CONSUME, MUSCLE_IN_PRODUCE];

                    for m in BrainMut::from_in_out(out_in_offset, out_in, false) {
                        // println!("{:?}", self.brain.as_ref().unwrap().get_structure());
                        // println!("{:?}", m);
                        self.mutate(&m);
                    }

                    self.body.muscles.remove(*muscle);
                    return true;
                }
            },
            Mutation::Brain(brain_mut) => {
                let no_brain = self.brain.is_none();
                if brain_mut.is_learn() && no_brain {
                    return false;
                }

                if no_brain {
                    self.brain = Some(Brain::default());
                }
                return self
                    .get_mut_organism()
                    .brain
                    .as_mut()
                    .unwrap()
                    .mutate(mutation);
            }
        }
    }
}
impl Organism {
    pub fn new(brain: Option<Brain>, body: Body, metabolism: Metabolism) -> Self {
        let metabolic_cost = (body
            .joints
            .iter()
            .map(|j| j.nodes.len() as f32)
            .sum::<f32>()
            * metabolism.node)
            + (body.joints.len() as f32 * metabolism.joint)
            + (body.bones.len() as f32 * metabolism.bone)
            + (body.muscles.len() as f32 * metabolism.muscle);

        Self {
            brain,
            body,
            static_stats: StaticStats::new(0.5),
            metabolic_cost,
        }
    }

    pub fn update_metabolic_cost(&mut self, metabolism: &Metabolism) {
        self.metabolic_cost = -(self
            .body
            .joints
            .iter()
            .map(|j| j.nodes.len() as f32)
            .sum::<f32>()
            * metabolism.node
            + (self.body.joints.len() as f32 * metabolism.joint)
            + (self.body.bones.len() as f32 * metabolism.bone)
            + (self.body.muscles.len() as f32 * metabolism.muscle));
    }

    pub fn max_energy(&self) -> f32 {
        return self.body.joints.len() as f32 * JOINT_MAX_ENERGY;
    }

    pub fn centre(&mut self) {
        let centre =
            self.body.joints.iter().map(|j| j.pos).sum::<Vec2>() / self.body.joints.len() as f32;

        for j in self.body.joints.iter_mut() {
            j.pos -= centre * 0.5;
        }
    }

    pub fn get_static_stats<'a>(&'a self) -> &'a StaticStats {
        return &self.static_stats;
    }

    pub fn as_seed(self, pos: Vec2) -> Seed {
        Seed::new(pos, self)
    }

    pub fn joint_out_in(&self, joint_index: usize) -> OutputConsumedInputProduced {
        // consts are reversed since we're talking about brain not node
        let mut out_in = OutputConsumedInputProduced([BASE_OUTPUT, BASE_INPUT]);
        for i in 0..joint_index {
            out_in += self.body.joints[i].nodes.iter().map(|n| n.out_in()).sum()
        }
        out_in
    }

    pub fn joint_node_out_in(
        &self,
        joint_index: usize,
        node_index: usize,
    ) -> OutputConsumedInputProduced {
        let mut out_in = self.joint_out_in(joint_index);
        // println!("{:?}", out_in);
        for i in 0..node_index {
            // println!("{i}, {:?}", out_in);
            out_in += self.body.joints[joint_index].nodes[i].out_in();
        }
        out_in
    }

    pub fn muscle_out_in(&self) -> OutputConsumedInputProduced {
        return self.joint_out_in(self.body.joints.len());
    }

    pub fn shift_edges(edges: &mut Vec<[usize; 2]>, index: &usize) {
        *edges = edges
            .iter()
            .filter(|[a, b]| a != index && b != index)
            .map(|[a, b]| {
                [
                    if a > index { a - 1 } else { *a },
                    if b > index { b - 1 } else { *b },
                ]
            })
            .collect::<Vec<[usize; 2]>>();
    }
}
impl OrganismAccessor for Organism {
    fn get_mut_organism<'a>(&'a mut self) -> &'a mut Organism {
        return self;
    }

    fn get_mut_body<'a>(&'a mut self) -> &'a mut Body {
        return &mut self.body;
    }

    fn get_mut_brain<'a>(&'a mut self) -> Option<&'a mut Brain> {
        return self.brain.as_mut();
    }

    fn get_organism<'a>(&'a self) -> &'a Organism {
        return &self;
    }

    fn get_body<'a>(&'a self) -> &'a Body {
        return &self.body;
    }

    fn get_brain<'a>(&'a self) -> &'a Option<Brain> {
        return &self.brain;
    }
}

mod tests {
    use bevy::math::vec2;

    use crate::{
        config::config::Metabolism,
        consts::{BASE_INPUT, BASE_OUTPUT, MUSCLE_IN_PRODUCE, MUSCLE_OUT_CONSUME},
        world::{
            environment::layer::layer_key::LayerKey,
            organism::{
                body::Body,
                brain::Brain,
                joint::Joint,
                node::{read::Read, thruster::Thruster, write::Write},
                node_type::NodeType,
                organism::Organism,
            },
        },
    };

    fn get_organism() -> Organism {
        Organism::new(
            Some(Brain::new(vec![2, 4, 1])),
            Body::new(
                vec![
                    Joint::new(
                        vec2(-5.0, 0.0),
                        vec![
                            NodeType::Thruster(Thruster::new(0.0)),
                            NodeType::Read(Read::new(LayerKey::Energy)),
                        ],
                    ),
                    Joint::new(
                        vec2(0.0, 6.0),
                        vec![NodeType::Write(Write::new(LayerKey::Energy))],
                    ),
                    Joint::new(vec2(5.0, 0.0), vec![]),
                ],
                vec![[0, 1], [1, 2]],
                vec![[0, 1]],
            ),
            Metabolism::default(),
        )
    }

    #[test]
    fn joint_out_in() {
        let o = get_organism();

        assert_eq!(
            Into::<[usize; 2]>::into(o.joint_out_in(0)),
            [BASE_OUTPUT, BASE_INPUT]
        );
        assert_eq!(
            Into::<[usize; 2]>::into(o.joint_out_in(1)),
            [BASE_OUTPUT + 1, BASE_INPUT + 1]
        );
        assert_eq!(
            Into::<[usize; 2]>::into(o.joint_out_in(2)),
            [BASE_OUTPUT + 2, BASE_INPUT + 1]
        );
    }

    #[test]
    fn joint_node_out_in() {
        let o = get_organism();

        assert_eq!(
            Into::<[usize; 2]>::into(o.joint_node_out_in(0, 0)),
            [BASE_OUTPUT, BASE_INPUT]
        );
        assert_eq!(
            Into::<[usize; 2]>::into(o.joint_node_out_in(0, 2)),
            [BASE_OUTPUT + 1, BASE_INPUT + 1]
        );
        assert_eq!(
            Into::<[usize; 2]>::into(o.joint_node_out_in(1, 0)),
            [BASE_OUTPUT + 1, BASE_INPUT + 1]
        );
        assert_eq!(
            Into::<[usize; 2]>::into(o.joint_node_out_in(2, 0)),
            [BASE_OUTPUT + 2, BASE_INPUT + 1]
        );
    }

    #[test]
    fn muscle_out_in() {
        let o = get_organism();

        assert_eq!(
            Into::<[usize; 2]>::into(o.muscle_out_in()),
            [
                BASE_OUTPUT + 2 + MUSCLE_OUT_CONSUME,
                BASE_INPUT + 1 + MUSCLE_IN_PRODUCE
            ]
        );
    }
}

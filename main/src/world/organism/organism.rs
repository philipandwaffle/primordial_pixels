use bevy::{core_pipeline::core_2d::graph::input, math::Vec2};
use serde::{Deserialize, Serialize};

use crate::{
    consts::{BASE_INPUT, BASE_OUTPUT, JOINT_RADIUS, MUSCLE_INPUTS, MUSCLE_OUTPUTS},
    world::organism::{
        body::Body,
        brain::Brain,
        in_out::OutputConsumedInputProduced,
        joint::Joint,
        mutation::{Body as BodyMut, Brain as BrainMut, Mutable, Mutation},
        seed::Seed,
        stats::StaticStats,
        util_trait::OrganismAccessor,
    },
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Organism {
    pub brain: Option<Brain>,
    pub body: Body,
    static_stats: StaticStats,
}
impl Mutable for Organism {
    fn mutate(&mut self, mutation: Mutation) -> bool {
        let mut o = self.clone();
        match mutation {
            Mutation::Body(body) => match body {
                BodyMut::AddNode { joint, node_type } => {
                    let out_in_offset = o.joint_out_in(joint).into();
                    let out_in = [MUSCLE_OUTPUTS, MUSCLE_INPUTS];

                    for m in BrainMut::from_in_out(out_in_offset, out_in, true) {
                        self.mutate(m);
                    }

                    // Add node
                    self.body.joints[joint].nodes.push(node_type);
                    return true;
                }
                BodyMut::AddJoint { pos } => {
                    // Get distances to node
                    let mut distances = self
                        .body
                        .joints
                        .iter()
                        .enumerate()
                        .map(|(i, j)| (i, pos.distance(j.pos)))
                        .filter(|(_, x)| *x > JOINT_RADIUS)
                        .collect::<Vec<(usize, f32)>>();
                    distances.sort_by(|(_, a), (_, b)| a.total_cmp(b));

                    if distances.is_empty() {
                        return false;
                    }

                    // Add joint
                    self.body.joints.push(Joint::new(pos, vec![]));

                    // Add shortest bone
                    self.mutate(Mutation::Body(BodyMut::AddBone {
                        bone: [distances[0].0, self.body.bones.len()],
                    }));
                    return true;
                }
                BodyMut::AddBone { bone } => {
                    self.body.bones.push(bone);
                    return true;
                }
                BodyMut::AddMuscle { muscle } => {
                    let out_in_offset = o.muscle_out_in().into();
                    let out_in = [MUSCLE_OUTPUTS, MUSCLE_INPUTS];

                    for m in BrainMut::from_in_out(out_in_offset, out_in, true) {
                        self.mutate(m);
                    }

                    self.body.muscles.push(muscle);
                    return true;
                }
                BodyMut::RemoveNode { joint, node } => {
                    let out_in_offset = o.joint_node_out_in(joint, node).into();
                    let out_in = self.body.joints[joint].nodes[node].out_con_in_prod().into();

                    for m in BrainMut::from_in_out(out_in_offset, out_in, false) {
                        self.mutate(m);
                    }

                    // remove node
                    self.body.joints[joint].nodes.remove(node);
                    return true;
                }
                BodyMut::RemoveJoint { joint: index } => {
                    self.body.joints.remove(index);
                    return true;
                }
                BodyMut::RemoveBone { bone } => {
                    self.body.bones.remove(bone);
                    return true;
                }
                BodyMut::RemoveMuscle { muscle } => {
                    let out_in_offset = o.muscle_out_in().into();
                    let out_in = [MUSCLE_OUTPUTS, MUSCLE_INPUTS];

                    for m in BrainMut::from_in_out(out_in_offset, out_in, false) {
                        self.mutate(m);
                    }

                    self.body.muscles.remove(muscle);
                    return true;
                }
            },
            Mutation::Brain(_) => {
                if let Some(b) = o.brain.as_mut() {
                    return b.mutate(mutation);
                }
                return false;
            }
        }
    }
}
impl Organism {
    pub fn new(brain: Option<Brain>, body: Body) -> Self {
        Self {
            brain,
            body,
            static_stats: StaticStats::new(1.0),
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
        let mut out_in = OutputConsumedInputProduced([BASE_INPUT, BASE_OUTPUT]);
        for i in 0..joint_index {
            out_in += self.body.joints[i]
                .nodes
                .iter()
                .map(|n| n.out_con_in_prod())
                .sum()
        }
        out_in
    }

    pub fn joint_node_out_in(
        &self,
        joint_index: usize,
        node_index: usize,
    ) -> OutputConsumedInputProduced {
        let mut out_in = self.joint_out_in(joint_index);
        println!("{:?}", out_in);
        for i in 0..node_index {
            println!("{i}, {:?}", out_in);
            out_in += self.body.joints[joint_index].nodes[i].out_con_in_prod();
        }
        out_in
    }

    pub fn muscle_out_in(&self) -> OutputConsumedInputProduced {
        return self.joint_out_in(self.body.joints.len())
            + OutputConsumedInputProduced([
                self.body.muscles.len() * MUSCLE_OUTPUTS,
                self.body.muscles.len() * MUSCLE_INPUTS,
            ]);
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
        consts::{BASE_INPUT, BASE_OUTPUT, MUSCLE_INPUTS, MUSCLE_OUTPUTS},
        world::organism::{
            body::Body,
            brain::Brain,
            joint::Joint,
            node_type::{NodeType, PheromoneRead, PheromoneWrite, Thruster},
            organism::Organism,
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
                            NodeType::PheromoneRead(PheromoneRead::new(0)),
                        ],
                    ),
                    Joint::new(
                        vec2(0.0, 6.0),
                        vec![NodeType::PheromoneWrite(PheromoneWrite::new(0))],
                    ),
                    Joint::new(vec2(5.0, 0.0), vec![]),
                ],
                vec![[0, 1], [1, 2]],
                vec![[0, 1]],
            ),
        )
    }

    #[test]
    fn joint_out_in() {
        let o = get_organism();

        assert_eq!(
            Into::<[usize; 2]>::into(o.joint_out_in(0)),
            [BASE_INPUT, BASE_OUTPUT]
        );
        assert_eq!(
            Into::<[usize; 2]>::into(o.joint_out_in(1)),
            [BASE_INPUT + 1, BASE_OUTPUT + 1]
        );
        assert_eq!(
            Into::<[usize; 2]>::into(o.joint_out_in(2)),
            [BASE_INPUT + 2, BASE_OUTPUT + 1]
        );
    }

    #[test]
    fn joint_node_out_in() {
        let o = get_organism();

        assert_eq!(
            Into::<[usize; 2]>::into(o.joint_node_out_in(0, 0)),
            [BASE_INPUT, BASE_OUTPUT]
        );
        assert_eq!(
            Into::<[usize; 2]>::into(o.joint_node_out_in(0, 2)),
            [BASE_INPUT + 1, BASE_OUTPUT + 1]
        );
        assert_eq!(
            Into::<[usize; 2]>::into(o.joint_node_out_in(1, 0)),
            [BASE_INPUT + 1, BASE_OUTPUT + 1]
        );
        assert_eq!(
            Into::<[usize; 2]>::into(o.joint_node_out_in(2, 0)),
            [BASE_INPUT + 2, BASE_OUTPUT + 1]
        );
    }

    #[test]
    fn muscle_out_in() {
        let o = get_organism();

        assert_eq!(
            Into::<[usize; 2]>::into(o.muscle_out_in()),
            [
                BASE_INPUT + 2 + MUSCLE_OUTPUTS,
                BASE_OUTPUT + 1 + MUSCLE_INPUTS
            ]
        );
    }
}

use std::collections::VecDeque;

use avian2d::prelude::{DistanceJoint, DistanceLimit};
use bevy::{
    app::{Plugin, Update},
    ecs::{
        query::{With, Without},
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    log::info,
    math::Quat,
    sprite_render::{ColorMaterial, MeshMaterial2d},
    time::Time,
    transform::components::Transform,
};

use crate::{
    assets::handles::{Handles, MatKey},
    config::config::{Organism as OrganismConfig, Transput as TransputConfig},
    consts::{KN, MUSCLE_Z, N},
    util::function::{quat_z_rot, rot_input},
    world::{
        environment::environment::Environment,
        organism::{
            component::{bone::Bone, joint::Joint, muscle::Muscle, organism::OrganismEntity},
            message::SpawnSeedMsg,
            node::node::Node,
            transput::{Transput, append_input},
            util_trait::OrganismAccessor,
        },
    },
};

pub struct OrganismPlugin;
impl Plugin for OrganismPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_message::<SpawnSeedMsg>().add_systems(
            Update,
            (
                Self::tick_organism_time,
                Self::update_brain_input,
                Self::update_brain_output,
                Self::update_muscles,
            )
                .chain(),
        );
    }
}
impl OrganismPlugin {
    fn tick_organism_time(time: Res<Time>, mut organisms: Query<&mut OrganismEntity>) {
        let dt = time.delta_secs();
        for mut o in organisms.iter_mut() {
            o.update_variable_stats(dt);
        }
    }

    fn update_brain_input(
        mut organisms: Query<&mut OrganismEntity>,
        mut joints: Query<(&mut Joint, &Transform)>,
        bone_query: Query<&Transform, With<Bone>>,
        mut muscle_query: Query<&mut Muscle>,
        env: Res<Environment<N, KN>>,
        transput_config: Res<TransputConfig>,
    ) {
        for mut organism_ent in organisms.iter_mut() {
            let mut energy = organism_ent.cur_energy;

            let mut input = match organism_ent.get_brain() {
                Some(b) => {
                    let mut input = VecDeque::with_capacity(b.get_num_inputs());

                    // base stimuli
                    let mb = organism_ent.get_static_stats().metronome_beat;
                    let beat = (organism_ent.get_variable_stats().time_alive % mb) / mb;

                    append_input(&mut input, beat);
                    input
                }
                None => VecDeque::with_capacity(0),
            };

            // joint input
            for joint_ent in organism_ent.joint_ents.iter() {
                if let Ok((mut j, t)) = joints.get_mut(*joint_ent) {
                    let pos = t.translation.truncate();
                    for node in j.nodes.iter_mut() {
                        node.produce_inputs(&mut energy, &mut input, &transput_config, (&env, pos));
                    }
                }
            }

            // muscle input
            for muscle_ent in organism_ent.muscle_ents.iter() {
                if let Ok(mut muscle) = muscle_query.get_mut(*muscle_ent) {
                    muscle.produce_inputs(&mut energy, &mut input, &transput_config, bone_query);
                }
            }
            // for j in organism_ent.get_body().muscles.iter() {
            //     if let Ok([trans_a, trans_b]) = bone_query
            //         .get_many([organism_ent.bone_ents[j[0]], organism_ent.bone_ents[j[1]]])
            //     {
            //         // input.push(rot_input(trans_a.rotation.angle_between(trans_b.rotation)));
            //         input.push(rot_input(quat_z_rot(trans_a.rotation)));
            //         input.push(rot_input(quat_z_rot(trans_b.rotation)));
            //     }
            // }

            if organism_ent.get_brain().is_some() {
                organism_ent.get_mut_brain().unwrap().set_input(input);
            }

            organism_ent.cur_energy = energy;
        }
    }

    fn update_brain_output(
        mut organisms: Query<&mut OrganismEntity>,
        mut joints: Query<(&mut Joint, &Transform)>,
        mut muscle: Query<&mut Muscle>,
        transput_config: Res<TransputConfig>,
        mut env: ResMut<Environment<N, KN>>,
    ) {
        for mut organism_ent in organisms.iter_mut() {
            let mut energy = organism_ent.cur_energy;

            let mut output = match organism_ent.get_brain() {
                Some(b) => b.process(),
                None => VecDeque::with_capacity(0),
            };

            // joint output
            for joint_ent in organism_ent.joint_ents.iter() {
                if let Ok((mut j, t)) = joints.get_mut(*joint_ent) {
                    let pos = t.translation.truncate();
                    for node in j.nodes.iter_mut() {
                        node.consume_outputs(
                            &mut energy,
                            &mut output,
                            &transput_config,
                            (&mut env, pos),
                        );
                    }
                }
            }

            // muscle output
            for m in organism_ent.muscle_ents.iter() {
                if let Ok(mut muscle) = muscle.get_mut(*m) {
                    muscle.consume_outputs(&mut energy, &mut output, &transput_config, ());
                }
            }

            organism_ent.cur_energy = energy;
        }
    }

    fn update_muscles(
        handles: Res<Handles>,
        mut muscles: Query<(
            &Muscle,
            &mut MeshMaterial2d<ColorMaterial>,
            &mut Transform,
            &mut DistanceJoint,
        )>,
        bones: Query<&Transform, (With<Bone>, Without<Muscle>)>,
    ) {
        for (m, mut mat, mut trans, mut dist_joint) in muscles.iter_mut() {
            // Update muscle length
            let muscle_length = m.get_absolute_len();
            dist_joint.limits = DistanceLimit::new(muscle_length, muscle_length);
            // dist_joint.with_limits(muscle_length, muscle_length);

            // Update muscle mat
            mat.0 = if m.get_cur_len() <= 0.2 {
                handles.get_mat_handle(&MatKey::Red)
            } else if m.get_cur_len() <= 0.4 {
                handles.get_mat_handle(&MatKey::Crimson)
            } else if m.get_cur_len() <= 0.6 {
                handles.get_mat_handle(&MatKey::Magenta)
            } else if m.get_cur_len() <= 0.8 {
                handles.get_mat_handle(&MatKey::Purple)
            } else {
                handles.get_mat_handle(&MatKey::Blue)
            };

            // Update muscle pos and rot
            if let Ok([trans_a, trans_b]) = bones.get_many([dist_joint.body1, dist_joint.body2]) {
                let pos_a = trans_a.translation.truncate();
                let pos_b: bevy::math::Vec2 = trans_b.translation.truncate();

                let dir = pos_a - pos_b;
                let z_rot = dir.y.atan2(dir.x);

                trans.translation = pos_a.midpoint(pos_b).extend(MUSCLE_Z);
                trans.rotation = Quat::from_rotation_z(z_rot);
                trans.scale.x = muscle_length;
            }
        }
    }
}

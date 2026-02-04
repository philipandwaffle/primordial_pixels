use avian2d::prelude::{DistanceJoint, DistanceLimit};
use bevy::{
    app::{Plugin, Update},
    ecs::{
        query::{With, Without},
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res},
    },
    log::info,
    math::Quat,
    sprite_render::{ColorMaterial, MeshMaterial2d},
    time::Time,
    transform::components::Transform,
};

use crate::{
    assets::handles::{Handles, MatKey},
    config::config::Organism as OrganismConfig,
    consts::MUSCLE_Z,
    util::function::{quat_z_rot, rot_input},
    world::organism::{
        component::{Bone, Joint, Muscle, OrganismEntity},
        message::SpawnSeedMsg,
        util_trait::OrganismAccessor,
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
        joints: Query<&Joint>,
        bones: Query<&Transform, With<Bone>>,
    ) {
        for mut organism_ent in organisms.iter_mut() {
            if let Some(b) = organism_ent.get_brain() {
                let mut input = Vec::with_capacity(b.get_num_inputs());

                // base stimuli
                let mb = organism_ent.get_static_stats().metronome_beat;
                let beat = (organism_ent.get_variable_stats().time_alive % mb) / mb;

                input.push(beat);

                // joint input todo!()
                // for j in organism_ent.get_body().joints.iter().flat_map(|j| &j.nodes) {}

                // muscle input
                for j in organism_ent.get_body().muscles.iter() {
                    if let Ok([trans_a, trans_b]) =
                        bones.get_many([organism_ent.bone_ents[j[0]], organism_ent.bone_ents[j[1]]])
                    {
                        // input.push(rot_input(trans_a.rotation.angle_between(trans_b.rotation)));
                        input.push(rot_input(quat_z_rot(trans_a.rotation)));
                        input.push(rot_input(quat_z_rot(trans_b.rotation)));
                    }
                }

                organism_ent.get_mut_brain().unwrap().set_input(input);
            }
        }
    }
    fn update_brain_output(
        commands: Commands,
        mut organisms: Query<&mut OrganismEntity>,
        mut joints: Query<&mut Joint>,
        mut muscle: Query<&mut Muscle>,
        organism_config: Res<OrganismConfig>,
    ) {
        for mut organism_ent in organisms.iter_mut() {
            if let Some(b) = organism_ent.get_brain() {
                // Process brain
                let mut output = b.process();
                organism_ent
                    .get_mut_brain()
                    .unwrap()
                    .set_output(output.clone());

                // joint output todo!()

                // muscle output
                let mut total_muscle_delta = 0.0;
                for m in organism_ent.muscle_ents.iter() {
                    if let Ok(mut muscle) = muscle.get_mut(*m) {
                        total_muscle_delta += muscle.set_len(output.pop().unwrap());
                    }
                }
                total_muscle_delta *= organism_config.muscle_efficiency;
            }
        }
    }

    // fn update_muscle_mats(
    //     handles: Res<Handles>,
    //     organisms: Query<&OrganismEntity>,
    //     mut muscles: Query<&mut MeshMaterial2d<ColorMaterial>, With<Muscle>>,
    // ) {
    //     for o in organisms.iter() {
    //         if let Some(b) = o.get_brain() {
    //             let mut output = b.get_output();
    //             for m_ent in o.muscle_ents.iter() {
    //                 if let Ok(mat) = muscles.get_mut(*m_ent).as_mut() {
    //                     if let Some(out) = output.pop() {
    //                         // let new_modifier = 1.0 + (out * 0.5);

    //                         m.set_len(out);
    //                         mat.0 = if out <= 0.2 {
    //                             handles.get_mat_handle(&MatKey::Red)
    //                         } else if out <= 0.4 {
    //                             handles.get_mat_handle(&MatKey::Crimson)
    //                         } else if out <= 0.6 {
    //                             handles.get_mat_handle(&MatKey::Magenta)
    //                         } else if out <= 0.8 {
    //                             handles.get_mat_handle(&MatKey::Purple)
    //                         } else {
    //                             handles.get_mat_handle(&MatKey::Blue)
    //                         };
    //                     } else {
    //                         panic!("");
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

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

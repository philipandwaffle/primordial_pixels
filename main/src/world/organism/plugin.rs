use std::{collections::VecDeque, f32::consts::PI};

use avian2d::prelude::{DistanceJoint, DistanceLimit, Forces, RigidBodyForces};
use bevy::{
    app::{First, Last, Plugin, PostUpdate, PreUpdate, Update},
    ecs::{
        entity::Entity,
        message::{MessageReader, MessageWriter},
        query::{With, Without},
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    log::info,
    math::{Quat, vec3},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    time::Time,
    transform::components::Transform,
};

use crate::{
    assets::handles::{Handles, MatKey},
    config::config::{Metabolism, Mutation as MutationConfig, Transput as TransputConfig},
    consts::{ENV_CELLS, KERNEL_CELLS, MUSCLE_Z, THRUSTER_BASE_LENGTH, THRUSTER_WIDTH, THRUSTER_Z},
    util::function::{quat_z_rot, rot_input, z_rot_to_dir},
    world::{
        environment::environment::Environment,
        organism::{
            component::{
                bone::Bone,
                egg::Egg,
                joint::{Joint, Thruster as ThrusterComp},
                muscle::Muscle,
                organism::OrganismMarker,
            },
            message::{SpawnEggMsg, SpawnOrganismMsg},
            node::thruster::Thruster,
            node_type::NodeType,
            transput::{Transput, append_input},
            util_trait::OrganismAccessor,
        },
    },
};

pub struct OrganismPlugin;
impl Plugin for OrganismPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_message::<SpawnEggMsg>()
            .add_message::<SpawnOrganismMsg>()
            .add_systems(
                First,
                (
                    Self::spawn_organism,
                    Self::spawn_egg,
                    Self::tick_organism_time,
                    Self::update_eggs,
                ),
            )
            .add_systems(PreUpdate, Self::update_brain_input)
            .add_systems(Update, Self::update_brain_output)
            .add_systems(PostUpdate, (Self::update_muscles, Self::update_thrusters));
        // .add_systems(Last, systems);
    }
}
impl OrganismPlugin {
    fn tick_organism_time(time: Res<Time>, mut organisms: Query<&mut OrganismMarker>) {
        let dt = time.delta_secs();
        for mut o in organisms.iter_mut() {
            o.update_variable_stats(dt);
            let energy = o.get_organism().meta.metabolic_cost * dt;
            o.update_energy(energy);
        }
    }

    fn update_brain_input(
        mut organisms: Query<&mut OrganismMarker>,
        mut joints: Query<(&mut Joint, &Transform)>,
        bone_query: Query<&Transform, With<Bone>>,
        mut muscle_query: Query<&mut Muscle>,
        time: Res<Time>,
        env: Option<Res<Environment<ENV_CELLS, KERNEL_CELLS>>>,
        transput_config: Res<TransputConfig>,
    ) {
        let dt = time.delta_secs();
        for mut organism_ent in organisms.iter_mut() {
            let mut energy = 0.0;

            let mut input = match organism_ent.get_brain() {
                Some(b) => {
                    let mut input = VecDeque::with_capacity(b.get_num_inputs());

                    // base stimuli
                    let mb = organism_ent.get_static_stats().metronome_beat;
                    let beat = (organism_ent.get_variable_stats().time_alive % mb) / mb;
                    let energy_level = organism_ent.get_energy_level();

                    // append_input(&mut input, beat);
                    append_input(&mut input, energy_level);

                    input
                }
                None => VecDeque::with_capacity(0),
            };

            // joint input
            if let Some(ref env) = env {
                for joint_ent in organism_ent.joint_ents.iter() {
                    if let Ok((mut j, t)) = joints.get_mut(*joint_ent) {
                        let pos = t.translation.truncate();
                        for node in j.nodes.iter_mut() {
                            node.produce_inputs(
                                &mut energy,
                                &mut input,
                                &transput_config,
                                (env, pos, dt),
                            );
                        }
                    }
                }
            }

            // muscle input
            for muscle_ent in organism_ent.muscle_ents.iter() {
                if let Ok(mut muscle) = muscle_query.get_mut(*muscle_ent) {
                    muscle.produce_inputs(&mut energy, &mut input, &transput_config, bone_query);
                }
            }

            if organism_ent.get_brain().is_some() {
                organism_ent.get_mut_brain().unwrap().set_input(input);
            }

            organism_ent.update_energy(energy);
        }
    }

    fn update_brain_output(
        mut organisms: Query<&mut OrganismMarker>,
        mut joints: Query<(&mut Joint, &Transform)>,
        mut muscle: Query<&mut Muscle>,
        transput_config: Res<TransputConfig>,
        time: Res<Time>,
        mut env: Option<ResMut<Environment<ENV_CELLS, KERNEL_CELLS>>>,
    ) {
        let dt = time.delta_secs();
        for mut organism_ent in organisms.iter_mut() {
            let mut energy = 0.0;

            let mut output = match organism_ent.get_brain() {
                Some(b) => b.process(),
                None => VecDeque::with_capacity(0),
            };

            // joint output
            if let Some(mut env) = env.as_mut() {
                for joint_ent in organism_ent.joint_ents.iter() {
                    if let Ok((mut j, t)) = joints.get_mut(*joint_ent) {
                        let pos = t.translation.truncate();
                        for node in j.nodes.iter_mut() {
                            node.consume_outputs(
                                &mut energy,
                                &mut output,
                                &transput_config,
                                (&mut env, pos, dt),
                            );
                        }
                    }
                }
            }

            // muscle output
            for m in organism_ent.muscle_ents.iter() {
                if let Ok(mut muscle) = muscle.get_mut(*m) {
                    muscle.consume_outputs(&mut energy, &mut output, &transput_config, ());
                }
            }

            organism_ent.update_energy(energy);
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

    fn update_thrusters(
        time: Res<Time>,
        transput_config: Res<TransputConfig>,
        mut joint_query: Query<(Forces, &Joint)>,
        mut thruster_query: Query<&mut Transform, With<ThrusterComp>>,
    ) {
        let dt = time.delta_secs();
        for (mut forces, joint) in joint_query.iter_mut() {
            if let Some(thruster_ent) = joint.thruster {
                let mut total_thrust = 0.0;
                let mut total_z_rot = 0.0;

                for node in joint.nodes.iter() {
                    if let NodeType::Thruster(t) = node {
                        total_thrust += t.thrust;
                        total_z_rot += t.z_rot;
                    }
                }
                total_z_rot = total_z_rot % PI;

                let thrust_vec = z_rot_to_dir(total_z_rot) * total_thrust;

                if let Ok(mut trans) = thruster_query.get_mut(thruster_ent) {
                    let visual_vec =
                        thrust_vec * THRUSTER_BASE_LENGTH / transput_config.thruster_strength;
                    trans.translation = (visual_vec * 0.5).extend(THRUSTER_Z);
                    trans.scale = vec3(THRUSTER_WIDTH, -visual_vec.length(), 1.0);
                    trans.rotation = Quat::from_rotation_z(-total_z_rot);
                }

                forces.apply_force(thrust_vec * dt);
            }
        }
    }

    fn spawn_egg(
        mut commands: Commands,
        mut spawn_egg_msg: MessageReader<SpawnEggMsg>,
        handles: Res<Handles>,
    ) {
        for msg in spawn_egg_msg.read() {
            println!("spawning egg");
            msg.spawn(&mut commands, &handles);
        }
    }

    fn update_eggs(
        mut commands: Commands,
        time: Res<Time>,
        mut egg_query: Query<(Entity, &mut Egg, &mut Transform)>,
        mut spawn_organism_msg: MessageWriter<SpawnOrganismMsg>,
    ) {
        let dt = time.delta_secs();
        for (ent, mut egg, mut trans) in egg_query.iter_mut() {
            if egg.tick(dt, &mut trans, &mut spawn_organism_msg) {
                commands.entity(ent).despawn();
            }
        }
    }

    fn spawn_organism(
        mut commands: Commands,
        mut spawn_organism_msg: MessageReader<SpawnOrganismMsg>,
        handles: Res<Handles>,
    ) {
        for msg in spawn_organism_msg.read() {
            msg.spawn(&mut commands, &handles);
        }
    }
}

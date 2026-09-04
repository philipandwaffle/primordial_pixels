use bevy::{
    app::{Plugin, Update},
    color::palettes::css::{GREEN, RED},
    ecs::{query::Without, system::Query},
    gizmos::gizmos::Gizmos,
    math::{Isometry3d, Quat, Vec3A, vec3, vec3a},
    transform::components::Transform,
};

use crate::{
    assets::handles::MatKey::Red,
    util::function::z_rot_to_dir,
    world::organism::{
        component::{joint::Joint, organism::OrganismMarker},
        node_type::NodeType,
    },
};

pub struct NodeDebugPlugin;
impl Plugin for NodeDebugPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, Self::debug_nodes);
    }
}
impl NodeDebugPlugin {
    fn debug_nodes(
        organisms: Query<&OrganismMarker>,
        joints: Query<(&Joint, &Transform), Without<OrganismMarker>>,
        mut gizmos: Gizmos,
    ) {
        for organism in organisms.iter() {
            for joint in organism.joint_ents.iter() {
                if let Ok((joint, joint_trans)) = joints.get(*joint) {
                    for node in joint.nodes.iter() {
                        match node {
                            NodeType::Energy(energy) => {}
                            NodeType::Decomposer(decomposer) => {}
                            NodeType::Read(read) => {
                                gizmos.circle(
                                    Isometry3d {
                                        rotation: Quat::IDENTITY,
                                        translation: Vec3A::from(
                                            joint_trans.translation
                                                + (z_rot_to_dir(read.z_rot + read.z_rot_offset)
                                                    * read.dist)
                                                    .extend(0.0),
                                        ),
                                    },
                                    0.01,
                                    GREEN,
                                );
                            }
                            NodeType::Write(write) => {}
                            NodeType::Thruster(thruster) => {}
                            NodeType::Spike(spike) => {}
                            NodeType::Eye(eye) => {
                                let num_rays = eye.get_num_rays();
                                let z_rot = eye.get_z_rot();
                                let fov = eye.get_fov();
                                let ray_dist = eye.get_ray_dist();

                                let step = fov / num_rays as f32;
                                let mut cur_z_rot = z_rot - (fov * 0.5) + (step * 0.5);

                                for mut dist in eye.get_hits() {
                                    if dist < 0.0 {
                                        dist = ray_dist;
                                    } else {
                                        dist *= ray_dist;
                                    }

                                    gizmos.circle(
                                        Isometry3d {
                                            rotation: Quat::from_rotation_z(-cur_z_rot),
                                            translation: Vec3A::from(
                                                joint_trans.translation + vec3(0.0, dist, 0.0),
                                            ),
                                        },
                                        0.05,
                                        RED,
                                    );
                                    cur_z_rot += step;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

use avian2d::prelude::{Forces, RigidBodyForces};
use bevy::{
    app::{First, Plugin, PostUpdate},
    ecs::{
        entity::Entity,
        message::MessageWriter,
        query::{Or, With, Without},
        schedule::And,
        system::{Query, Res, ResMut},
    },
    log::info,
    math::{Vec2, Vec3, vec2},
    transform::components::Transform,
};
use my_derive::ConfigTag;
use rand::{Rng, rng};
use rand_distr::num_traits::Pow;
use serde::{Deserialize, Serialize};

use crate::{
    config::{
        config::{Metabolism, Mutation as MutationConfig, Storage},
        config_tag::ConfigTag,
    },
    petri_dish::resource::PetriDishInfo,
    world::{
        environment::plugin::EnvironmentPlugin,
        organism::{
            component::{bone::Bone, egg::Egg, joint::Joint, organism::OrganismMarker},
            message::{DespawnOrganismMsg, SpawnEggMsg, SpawnOrganismMsg},
            seed::Seed,
        },
    },
};

#[derive(ConfigTag, Serialize, Deserialize, Clone)]
pub struct PetriDishPlugin {
    pub init_seed: Option<Seed>,
    pub min_organisms: usize,
    pub initial_num_mutations: usize,
    pub num_mutations: usize,
    pub boundary_width: f32,
    pub side_len: f32,
    pub display_update_interval: f32,
}
impl Plugin for PetriDishPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(PetriDishInfo::new(
            self.init_seed.clone(),
            self.min_organisms,
            self.initial_num_mutations,
            self.num_mutations,
            self.boundary_width,
            self.side_len,
        ))
        .add_plugins(EnvironmentPlugin::new(self.display_update_interval))
        .add_systems(First, Self::replenish_organisms);
        app.add_systems(PostUpdate, (Self::evaluate_organisms, Self::warp));
    }
}

impl PetriDishPlugin {
    fn nudge(
        info: Res<PetriDishInfo>,
        mut joint_query: Query<(Forces, &Transform), Or<(With<Joint>, With<Egg>)>>,
    ) {
        for (mut forces, trans) in joint_query.iter_mut() {
            let pos = trans.translation.truncate();

            let x_abs = pos.x.abs();
            let y_abs = pos.y.abs();

            let mut nudge = Vec2::ZERO;
            if x_abs > info.threshold {
                nudge.x = (info.threshold * pos.x.signum()) - pos.x;
            }
            if y_abs > info.threshold {
                nudge.y = (info.threshold * pos.y.signum()) - pos.y;
            }

            if nudge != Vec2::ZERO {
                forces.apply_force(nudge);
            }
        }
    }

    fn warp(
        info: Res<PetriDishInfo>,
        mut joints: Query<&mut Transform, With<Joint>>,
        mut bones: Query<&mut Transform, (With<Bone>, Without<Joint>)>,
        organisms: Query<&OrganismMarker>,
    ) {
        for o in organisms.iter() {
            let pos = o.get_pos_from_mut(&joints);

            let delta = (info.threshold * 2.0) - info.boundary_width * 0.5;
            let offset = match (pos.x.abs() > info.threshold, pos.y.abs() > info.threshold) {
                (true, true) => vec2(delta * pos.x.signum(), delta * pos.y.signum()),
                (true, false) => vec2(delta * pos.x.signum(), 0.0),
                (false, true) => vec2(0.0, delta * pos.y.signum()),
                _ => continue,
            };

            for j_ent in o.joint_ents.iter() {
                if let Ok(mut j_trans) = joints.get_mut(*j_ent) {
                    j_trans.translation -= offset.extend(0.0);
                }
            }
            for b_ent in o.bone_ents.iter() {
                if let Ok(mut b_trans) = bones.get_mut(*b_ent) {
                    b_trans.translation -= offset.extend(0.0);
                }
            }
        }
    }

    fn replenish_organisms(
        mut spawn_organism_msg: MessageWriter<SpawnOrganismMsg>,
        mut info: ResMut<PetriDishInfo>,
        mutation_config: Res<MutationConfig>,
        metabolism: Res<Metabolism>,
        storage: Res<Storage>,
    ) {
        // Guarantee mutation occurs on spawn
        let mut mutation_config = mutation_config.clone();
        mutation_config.rate = 1.0;
        mutation_config.learn_rate = 1.0;
        mutation_config.learn_factor = 1.0;

        let to_spawn = info
            .min_organisms
            .checked_sub(info.cur_organisms)
            .unwrap_or(0);
        if to_spawn == 0 {
            return;
        }
        info!(
            "Spawning {} organisms {}/{}",
            to_spawn, info.cur_organisms, info.min_organisms
        );

        let mut rng = rng();
        let s = info.init_seed.clone();
        for _ in 0..to_spawn {
            let pos = vec2(
                rng.random_range(-info.threshold..=info.threshold),
                rng.random_range(-info.threshold..=info.threshold),
            );
            // let pos = vec2(0.0, 0.0);
            let mut s = s.clone();
            s.set_pos(pos);
            s.multi_mutate(
                &mut rng,
                &metabolism,
                &storage,
                &mutation_config,
                info.initial_num_mutations,
            );

            spawn_organism_msg.write(Into::<SpawnOrganismMsg>::into(s));
        }
        info.cur_organisms += to_spawn;
    }

    fn evaluate_organisms(
        mut info: ResMut<PetriDishInfo>,
        metabolism: Res<Metabolism>,
        storage: Res<Storage>,
        mutation_config: Res<MutationConfig>,
        mut spawn_egg_msg: MessageWriter<SpawnEggMsg>,
        mut despawn_organism_msg: MessageWriter<DespawnOrganismMsg>,
        mut organism_query: Query<(Entity, &mut OrganismMarker)>,
        joint_query: Query<&Transform, With<Joint>>,
    ) {
        let mut rng = rng();
        for (ent, mut organism) in organism_query.iter_mut() {
            if organism.is_dead() {
                despawn_organism_msg.write(DespawnOrganismMsg::new(ent));
                info.cur_organisms -= 1;
            } else if let Some(mut s) = organism.reproduce(&metabolism, &joint_query) {
                info.cur_organisms += 1;

                s.multi_mutate(
                    &mut rng,
                    &metabolism,
                    &storage,
                    &mutation_config,
                    info.num_mutations,
                );

                spawn_egg_msg.write(Into::<SpawnEggMsg>::into(s));
            }
        }
    }
}

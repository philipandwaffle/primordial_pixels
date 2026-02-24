use avian2d::prelude::{Forces, RigidBody, RigidBodyForces};
use bevy::{
    app::{First, Last, Plugin, PostUpdate, Update},
    ecs::{
        entity::Entity,
        message::MessageWriter,
        query::{Or, With},
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    log::info,
    math::{NormedVectorSpace, Vec2, vec2},
    transform::components::Transform,
};
use my_derive::ConfigTag;
use rand::{Rng, rng};
use rand_distr::num_traits::{CheckedSub, Pow};
use serde::{Deserialize, Serialize};

use crate::{
    config::{
        config::{Metabolism, Mutation as MutationConfig},
        config_tag::ConfigTag,
    },
    consts::JOINT_RADIUS,
    petri_dish::resource::PetriDishInfo,
    util::function::rand_vec2,
    world::{
        environment::plugin::EnvironmentPlugin,
        organism::{
            component::{egg::Egg, joint::Joint, organism::OrganismMarker},
            message::{DespawnOrganismMsg, SpawnEggMsg, SpawnOrganismMsg},
            mutation::mutation::{Mut, Mutable, Mutation},
            seed::{self, Seed},
            util_trait::OrganismAccessor,
        },
    },
};

#[derive(ConfigTag, Serialize, Deserialize, Clone)]
pub struct PetriDishPlugin {
    pub init_seed: Option<Seed>,
    pub min_organisms: usize,
    pub initial_num_mutations: usize,
    pub num_mutations: usize,
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
            self.side_len,
        ))
        .add_plugins(EnvironmentPlugin::new(
            self.side_len,
            self.display_update_interval,
        ))
        .add_systems(First, Self::replenish_organisms);
        app.add_systems(PostUpdate, (Self::evaluate_organisms, Self::nudge));
    }
}

impl PetriDishPlugin {
    fn nudge(
        info: Res<PetriDishInfo>,
        mut joint_query: Query<(Forces, &Transform), Or<(With<Joint>, With<Egg>)>>,
    ) {
        for (mut forces, trans) in joint_query.iter_mut() {
            let pos = trans.translation.truncate();

            let x = pos.x.abs();
            let y = pos.y.abs();

            if x > info.threshold || y > info.threshold {
                let dist = x.max(y) - info.threshold;

                forces.apply_force(-pos.normalize() * (1.0 + dist.pow(2.0)));
            }
        }
    }

    fn replenish_organisms(
        mut spawn_organism_msg: MessageWriter<SpawnOrganismMsg>,
        mut info: ResMut<PetriDishInfo>,
        mutation_config: Res<MutationConfig>,
        metabolism: Res<Metabolism>,
    ) {
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
                rng.random_range(-info.half_side_len..=info.half_side_len),
                rng.random_range(-info.half_side_len..=info.half_side_len),
            );
            // let pos = vec2(0.0, 0.0);
            let mut s = s.clone();
            s.set_pos(pos);
            s.multi_mutate(
                &mut rng,
                &metabolism,
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

                s.multi_mutate(&mut rng, &metabolism, &mutation_config, info.num_mutations);

                spawn_egg_msg.write(Into::<SpawnEggMsg>::into(s));
            }
        }
    }
}

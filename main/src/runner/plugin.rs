use std::vec;

use avian2d::prelude::{Collider, RigidBody};
use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    ecs::{
        entity::Entity,
        message::MessageWriter,
        query::{With, Without},
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    log::{info, trace},
    math::{Vec2, VectorSpace, primitives::Rectangle, vec2},
    mesh::{Mesh, Mesh2d},
    sprite_render::MeshMaterial2d,
    time::Time,
    transform::components::Transform,
};

use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};

use crate::{
    assets::handles::{Handles, MatKey},
    config::config::Organism as OrganismConfig,
    consts::JOINT_RADIUS,
    organism_logger::LogOrganismsEvent,
    world::organism::{
        component::{Joint, OrganismEntity},
        mutation::{Mut, Mutable, Mutation},
        organism::Organism,
        seed::Seed,
        util_trait::OrganismAccessor,
    },
};

#[derive(Serialize, Deserialize)]
pub struct RunnerPlugin {
    pub seed: Option<Seed>,
    pub num_organisms: usize,
    pub initial_num_mutations: usize,
    pub num_mutations: usize,
    pub generation_duration: f32,
    pub cage_size: Vec2,
    pub save_interval: usize,
    pub cur_generation: Option<usize>,
}
impl Plugin for RunnerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let s = if let Some(s) = self.seed.clone() {
            s
        } else {
            Seed::default()
        };

        app.insert_resource(Generation::new(
            s,
            self.num_organisms,
            self.initial_num_mutations,
            self.num_mutations,
            self.cage_size,
            self.generation_duration,
            self.save_interval,
            self.cur_generation,
        ))
        .add_systems(Startup, Self::init_generation)
        .add_systems(
            Update,
            (
                Self::tick_timer,
                Self::spawn_next_generation.run_if(Self::is_time_up),
            )
                .chain(),
        );
    }
}
impl RunnerPlugin {
    fn tick_timer(time: Res<Time>, mut generation: ResMut<Generation>) {
        if generation.timer.time_up() {
            return;
        }
        generation.timer.tick(time.delta_secs());
    }

    fn is_time_up(generation: Res<Generation>) -> bool {
        return generation.timer.time_up();
    }

    fn spawn_next_generation(
        mut commands: Commands,
        mut generation: ResMut<Generation>,
        organism_config: Res<OrganismConfig>,
        handles: Res<Handles>,
        organisms: Query<(Entity, &OrganismEntity), Without<Joint>>,
        joints: Query<&Transform, With<Joint>>,
        mut log_msg: MessageWriter<LogOrganismsEvent>,
    ) {
        let mut seeds = Vec::with_capacity(generation.num_organisms);
        for (o_ent, organism_ent) in organisms.iter() {
            let mut pos = Vec2::ZERO;
            for j_ent in organism_ent.joint_ents.iter() {
                match joints.get(*j_ent) {
                    Ok(t) => {
                        let joint_pos = t.translation.truncate();
                        if joint_pos.x > pos.x {
                            pos.x = joint_pos.x
                        }
                    }
                    Err(e) => panic!("Cannot get joint {e}"),
                }
            }
            let num_joints_f32 = organism_ent.joint_ents.len() as f32;
            let num_muscles = organism_ent.muscle_ents.len() as f32;
            // pos /= num_joints_f32;

            // let fitness = (centre.x.abs()).powf(2.0) - (num_joints_f32.powf(2.0))
            //     + (o.muscle_ents.len() as f32).powf(2.0);
            // let fitness = centre.x.abs().powf(1.4) - num_joints_f32.powf(2.0);
            let fitness = if pos.x < 0.0 {
                0.0
            } else {
                pos.x.powf(1.4) + num_muscles.powf(1.3)
            };
            seeds.push((organism_ent.as_seed(Vec2::ZERO), fitness));
            commands.entity(o_ent).despawn();
        }
        generation.organisms.clear();

        seeds.sort_by(|(_, a), (_, b)| {
            match (b).partial_cmp(a) {
                Some(ord) => ord,
                None => match (a.is_nan(), b.is_nan()) {
                    (true, true) => std::cmp::Ordering::Equal,
                    (true, _) => std::cmp::Ordering::Greater,
                    (_, true) => std::cmp::Ordering::Less,
                    (_, _) => std::cmp::Ordering::Equal, // should never happen
                },
            }
        });

        info!(
            "fitnesses: {:?}",
            seeds.iter().map(|x| x.1).collect::<Vec<f32>>()
        );
        info!(
            "best fitness of {}, average: {}",
            seeds[0].1,
            seeds.iter().map(|x| x.1).sum::<f32>() / generation.num_organisms as f32
        );

        let mut cur_pos = Vec2::ZERO;
        let normal = Normal::new(0.0, (generation.num_organisms as f32).powf(0.8)).unwrap();
        let mut picked = vec![];

        let mut id = 0;
        let num_muts = generation.num_mutations;
        let mut rng = rand::rng();
        while generation.organisms.len() < generation.num_organisms {
            let i = normal.sample(&mut rand::rng()).abs() as usize;
            let i = i.clamp(0, generation.num_organisms - 1);

            picked.push(i);
            generation.organisms.push(Self::spawn_seed(
                &mut commands,
                &mut rng,
                seeds[i].0.clone(),
                cur_pos,
                &id.to_string(),
                num_muts,
                &handles,
                &organism_config,
            ));

            cur_pos.y += generation.cage_size.y;
            id += 1;
        }
        trace!("Picked: {picked:?}");

        if generation.cur_generation % generation.save_interval == 0 {
            log_msg.write(LogOrganismsEvent::new(
                seeds.iter().map(|(s, _)| s.clone()).collect(),
                format!("generation/{}", generation.cur_generation),
            ));
        }
        generation.cur_generation += 1;
        generation.timer.reset();
    }

    fn init_generation(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        handles: Res<Handles>,
        generation: ResMut<Generation>,
        organism_config: Res<OrganismConfig>,
    ) {
        let mut organisms = Vec::with_capacity(generation.num_organisms);
        let mut cur_pos = Vec2::ZERO;

        let mesh = meshes.add(Rectangle::new(generation.cage_size.x, 1.0));
        let thickness = 1.0;
        let mut rng = rand::rng();
        for id in 0..generation.num_organisms {
            organisms.push(Self::spawn_seed(
                &mut commands,
                &mut rng,
                generation.init_seed.clone(),
                cur_pos,
                &id.to_string(),
                generation.initial_num_mutations,
                &handles,
                &organism_config,
            ));

            commands.spawn((
                RigidBody::Static,
                Collider::rectangle(generation.cage_size.x, thickness),
                Mesh2d(mesh.clone()),
                handles.get_mat2d(&MatKey::Red),
                Transform::from_xyz(0.0, cur_pos.y - (thickness * 0.5), -1.0),
            ));

            cur_pos.y += generation.cage_size.y;
        }
    }

    fn spawn_seed(
        commands: &mut Commands,
        rng: &mut ThreadRng,
        mut s: Seed,
        spawn_pos: Vec2,
        name: &str,
        num_muts: usize,
        handles: &Handles,
        oc: &OrganismConfig,
    ) -> Entity {
        for _ in 0..num_muts {
            let o = s.get_organism();
            if let Some(mutation) = Mutation::rand(rng, oc, o) {
                s.mutate(mutation);
            }
        }

        let mut offset = vec2(0.0, 0.0);
        for j in s.get_body().joints.iter() {
            if offset.y > j.pos.y {
                offset.y = j.pos.y;
            }
            offset.x += j.pos.x
        }
        offset.y -= JOINT_RADIUS;
        offset.x /= s.get_body().joints.len() as f32;
        s.set_pos(spawn_pos + offset);

        s.spawn(commands, handles)
    }
}

#[derive(Resource)]
struct Generation {
    init_seed: Seed,
    organisms: Vec<Entity>,
    num_organisms: usize,
    initial_num_mutations: usize,
    num_mutations: usize,
    cage_size: Vec2,
    timer: Timer,
    save_interval: usize,
    cur_generation: usize,
}
impl Generation {
    pub fn new(
        seed: Seed,
        num_organisms: usize,
        initial_num_mutations: usize,
        num_mutations: usize,
        cage_size: Vec2,
        duration: f32,
        save_interval: usize,
        cur_generation: Option<usize>,
    ) -> Self {
        return Self {
            init_seed: seed,
            organisms: vec![],
            num_organisms,
            initial_num_mutations,
            num_mutations,
            cage_size,
            timer: Timer::new(duration),
            save_interval,
            cur_generation: cur_generation.unwrap_or(0),
        };
    }
}

struct Timer {
    cur_time: f32,
    duration: f32,
}
impl Timer {
    pub fn new(duration: f32) -> Self {
        return Self {
            cur_time: 0.0,
            duration,
        };
    }
    pub fn tick(&mut self, dt: f32) {
        self.cur_time += dt;
    }

    pub fn time_up(&self) -> bool {
        return self.cur_time >= self.duration;
    }
    pub fn reset(&mut self) {
        self.cur_time = 0.0;
    }
}

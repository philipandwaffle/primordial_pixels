use bevy::{
    app::Plugin,
    ecs::{
        query::With,
        system::{Commands, Query, Res},
    },
    transform::components::Transform,
};

use crate::{
    config::config::Organism as OrganismConfig,
    world::organism::{
        component::{Bone, Joint, Muscle, Organism},
        message::SpawnSeedMsg,
    },
};

pub struct OrganismPlugin;
impl Plugin for OrganismPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_message::<SpawnSeedMsg>();
    }
}
impl OrganismPlugin {
    fn update_brains(
        commands: Commands,
        mut organisms: Query<&mut Organism>,
        mut joints: Query<&mut Joint>,
        bones: Query<(&Transform), With<Bone>>,
        mut muscle: Query<&mut Muscle>,
        organism_config: Res<OrganismConfig>,
    ) {
        for mut o in organisms.iter_mut() {
            if let Some(b) = o.brain.as_ref() {
                let mut input = Vec::with_capacity(b.get_num_inputs());

                for j in o.muscle_bones.iter() {
                    if let Ok([trans_a, trans_b]) = bones.get_many([o.bones[j[0]], o.bones[j[1]]]) {
                        input.push(trans_a.rotation.angle_between(trans_b.rotation));
                    }
                }

                let mut output = b.process(input);

                let mut total_muscle_delta = 0.0;
                for m in o.muscles.iter() {
                    if let Ok(mut muscle) = muscle.get_mut(*m) {
                        total_muscle_delta += muscle.set_len(1.0 + (output.pop().unwrap() * 0.5));
                    }
                }
                total_muscle_delta *= organism_config.muscle_efficiency;
            }
        }
    }
}

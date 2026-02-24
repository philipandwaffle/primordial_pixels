use std::path::Path;

use bevy::ecs::message::Message;

use crate::{
    config::{config_tag::Config, plugin::load_config},
    save::seed_packet::SeedPacket,
    world::{environment::environment::Environment, organism::seed::Seed},
};

#[derive(Message)]
pub struct SaveMsg<const N: usize, const KN: usize> {
    pub(crate) seeds: Vec<Seed>,
    pub(crate) env: Environment<N, KN>,
    pub(crate) path: String,
}
impl<const N: usize, const KN: usize> SaveMsg<N, KN> {
    pub fn new(seeds: Vec<Seed>, env: Environment<N, KN>, path: String) -> Self {
        return Self { seeds, env, path };
    }
}
impl<const N: usize, const KN: usize> Config for SaveMsg<N, KN> {
    fn load_cfg(_: &Path) -> Self {
        panic!("Cannot load a save message");
    }

    fn save_cfg(&self, path: &Path) {
        let path_buff = path.to_path_buf();

        SeedPacket::new(self.seeds.clone()).save_cfg(path_buff.join("seeds").as_path());
        self.env.save_cfg(path_buff.join("env.json").as_path());
    }
}

#[derive(Message)]
pub struct LoadMsg<const N: usize, const KN: usize> {
    pub(crate) seeds: Vec<Seed>,
    pub(crate) env: Environment<N, KN>,
}
impl<const N: usize, const KN: usize> Config for LoadMsg<N, KN> {
    fn load_cfg(path: &Path) -> Self {
        let path_buff = path.to_path_buf();

        let seed_packet = SeedPacket::load_cfg(path_buff.join("seeds").as_path());
        let env = Environment::<N, KN>::load_cfg(path_buff.join("env.json").as_path());

        Self {
            seeds: seed_packet.seeds,
            env,
        }
    }

    fn save_cfg(&self, _: &Path) {
        panic!("Cannot save a load message");
    }
}

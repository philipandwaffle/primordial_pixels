use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct SaveInfo {
    pub(crate) log_dir: String,
}
impl SaveInfo {
    pub fn new(log_dir: String) -> Self {
        Self { log_dir }
    }
}

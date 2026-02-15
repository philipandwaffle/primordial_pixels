use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct SaveInfo {
    pub(crate) log_dir: String,
    pub(crate) load_dir: Option<String>,
}
impl SaveInfo {
    pub fn new(log_dir: String, load_dir: Option<String>) -> Self {
        Self { log_dir, load_dir }
    }
}

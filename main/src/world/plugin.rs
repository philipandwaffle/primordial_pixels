use bevy::app::Plugin;

use crate::world::organism::plugin::OrganismPlugin;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(OrganismPlugin);
    }
}
impl WorldPlugin {}

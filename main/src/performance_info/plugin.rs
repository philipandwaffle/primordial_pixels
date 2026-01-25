use bevy::{
    app::Plugin,
    diagnostic::{
        FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
    },
};

pub struct PerformanceInfoPlugin;
impl Plugin for PerformanceInfoPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(LogDiagnosticsPlugin::default());
        app.add_plugins(SystemInformationDiagnosticsPlugin::default());
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    }
}
impl PerformanceInfoPlugin {}

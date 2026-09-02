use bevy::{
    app::Plugin,
    diagnostic::{
        FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
    },
};

pub struct PerformanceDebugPlugin;
impl Plugin for PerformanceDebugPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(LogDiagnosticsPlugin::default());
        app.add_plugins(SystemInformationDiagnosticsPlugin::default());
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    }
}
impl PerformanceDebugPlugin {}

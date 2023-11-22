use bevy::prelude::*;

pub struct MyPlugins;

impl Plugin for MyPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            crate::game::player::PlayerPlugin,
            crate::game::settings::SettingsPlugin,
            crate::game::camera::CameraPlugin,
            bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
        ));
    }
}

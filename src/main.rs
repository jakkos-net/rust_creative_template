use bevy::{DefaultPlugins, prelude::App};
use bevy_egui::EguiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .run();
}

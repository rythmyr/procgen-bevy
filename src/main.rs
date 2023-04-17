#![deny(clippy::all)]
#![deny(clippy::panic)]

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use player::Player;

use crate::{
    chunks_plugin::ChunksPlugin,
    controls_plugin::{ControlsPlugin, MainCamera},
};

mod chunks_plugin;
mod controls_plugin;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(ControlsPlugin)
        .add_plugin(ChunksPlugin)
        .add_startup_system(startup_system)
        .run();
}

fn startup_system(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-10.0, -10.0, -10.0)
                .looking_at(Vec3::from((0.0, -10.0, -10.0)), Vec3::from((0.0, 1.0, 0.0))),
            ..Default::default()
        },
        MainCamera {},
        Player {},
    ));

    commands.insert_resource(AmbientLight::default());
}

#![deny(clippy::all)]
#![deny(clippy::panic)]

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

use crate::controls_plugin::{ControlsPlugin, MainCamera};
use noise::{NoiseFn, Perlin};
use rand::prelude::*;

mod controls_plugin;

const CHUNK_SIZE: u8 = 16u8;
const NUM_CHUNKS: u8 = 4u8;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(ControlsPlugin)
        .add_startup_system(startup_system)
        .run();
    println!("Hello, world!");
}

fn generate_chunks(
    commands: &mut Commands,
    perlin_noise: &Perlin,
    cube_handle: Handle<Mesh>,
    cube_material_handle: Handle<StandardMaterial>,
    chunk_x: u8,
    chunk_y: u8,
    chunk_z: u8,
) {
    for x in 0..CHUNK_SIZE {
        let fx = x as f64 + (chunk_x as f64 * CHUNK_SIZE as f64);
        for y in 0..CHUNK_SIZE {
            let fy = y as f64 + (chunk_y as f64 * CHUNK_SIZE as f64);
            for z in 0..CHUNK_SIZE {
                let fz = z as f64 + (chunk_z as f64 * CHUNK_SIZE as f64);
                let noise_value = perlin_noise.get([fx / 16., fy / 16., fz / 16.]);
                let bundle = (PbrBundle {
                    mesh: cube_handle.clone(),
                    material: cube_material_handle.clone(),
                    transform: Transform::from_xyz(fx as f32, fy as f32, fz as f32),
                    ..default()
                },);
                if noise_value <= 0.0 {
                    commands.spawn(bundle);
                } else {
                    // println!("skipping cube!, {:?}, {}", (fx, fy, fz), noise_value);
                }
            }
        }
    }
}

fn startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let seed = random::<u32>();
    let perlin_noise = Perlin::new(seed);
    let i = 0.0;
    let j = 0.0;
    let k = 0.0;
    perlin_noise.get([i, j, k]);

    let cube_handle = meshes.add(Mesh::from(shape::Cube {
        ..Default::default()
    }));

    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::Rgba {
            red: 0.8,
            green: 0.2,
            blue: 1.0,
            alpha: 1.0,
        },

        ..Default::default()
    });

    for x in 0..NUM_CHUNKS {
        for y in 0..NUM_CHUNKS {
            for z in 0..NUM_CHUNKS {
                generate_chunks(
                    &mut commands,
                    &perlin_noise,
                    cube_handle.clone(),
                    cube_material_handle.clone(),
                    x,
                    y,
                    z,
                );
            }
        }
    }

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-10.0, -10.0, -10.0)
                .looking_at(Vec3::ZERO, Vec3::from((0.0, 1.0, 0.0))),
            ..Default::default()
        },
        MainCamera {},
    ));

    commands.spawn(DirectionalLightBundle { ..default() });
}

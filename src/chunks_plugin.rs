use bevy::{prelude::*, utils::HashSet};

use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::player::Player;

const CHUNK_SIZE: isize = 16isize;
// const BLOCKS_PER_CHUNK: usize = (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize;

const RENDER_DISTANCE_CHUNKS: isize = 3;
const RENDER_DISTANCE_UNITS: isize = RENDER_DISTANCE_CHUNKS * CHUNK_SIZE;

pub struct ChunksPlugin;

#[derive(Resource)]
pub struct PerlinInfo {
    generator: Perlin,
}

impl Plugin for ChunksPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_resources)
            .add_system(chunk_should_load_check)
            .add_system(generate_chunks);
    }
}

#[derive(Resource)]
pub struct CubeHandle(Handle<Mesh>);

#[derive(Resource)]
pub struct CubeMaterialHandle(Handle<StandardMaterial>);

fn add_resources(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let seed: u32 = rand::thread_rng().gen();
    let perlin: Perlin = Perlin::new(seed);

    let perlin_info = PerlinInfo { generator: perlin };

    commands.insert_resource(perlin_info);

    let cube_handle = meshes.add(Mesh::from(shape::Cube {
        ..Default::default()
    }));
    commands.insert_resource(CubeHandle(cube_handle));

    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::Rgba {
            red: 0.8,
            green: 0.2,
            blue: 1.0,
            alpha: 1.0,
        },
        ..Default::default()
    });

    commands.insert_resource(CubeMaterialHandle(cube_material_handle));
}

fn chunk_should_load_check(
    players: Query<&Transform, With<Player>>,
    chunks: Query<(&Chunk, Entity)>, //
    mut commands: Commands,
    material: Res<CubeMaterialHandle>,
    cube: Res<CubeHandle>,
) {
    let cube_handle = &cube.0;
    let cube_material_handle = &material.0;
    let mut existing_chunks = HashSet::<(isize, isize, isize)>::new();
    for tuple in chunks.iter() {
        let (chunk, entity) = tuple;
        existing_chunks.insert((chunk.x, chunk.y, chunk.z));
        for player in players.iter() {
            if (chunk.center() - player.translation).length_squared()
                >= (RENDER_DISTANCE_UNITS * RENDER_DISTANCE_UNITS) as f32
                && chunk.load_state != LoadState::ShouldUnload
            {
                // NOTE: not really doing anything with the loadstate yet
                // It'll be really useful when we have saving/loading
                commands.entity(entity).despawn();
            }
        }
    }

    for player in players.iter() {
        let px = player.translation.x.floor() as isize / CHUNK_SIZE;
        let py = player.translation.y.floor() as isize / CHUNK_SIZE;
        let pz = player.translation.z.floor() as isize / CHUNK_SIZE;

        for x in px - RENDER_DISTANCE_CHUNKS..px + RENDER_DISTANCE_CHUNKS {
            for y in py - RENDER_DISTANCE_CHUNKS..py + RENDER_DISTANCE_CHUNKS {
                for z in pz - RENDER_DISTANCE_CHUNKS..pz + RENDER_DISTANCE_CHUNKS {
                    let chunk = Chunk {
                        generation_state: GenerationState::NotStarted,
                        // blocks: [0; BLOCKS_PER_CHUNK],
                        x,
                        y,
                        z,
                        load_state: LoadState::ShouldLoad,
                    };
                    if (chunk.center() - player.translation).length_squared()
                        < (RENDER_DISTANCE_UNITS * RENDER_DISTANCE_UNITS) as f32
                        && !existing_chunks.contains(&(x, y, z))
                    {
                        commands.spawn((
                            chunk,
                            PbrBundle {
                                mesh: cube_handle.clone(),
                                material: cube_material_handle.clone(),
                                transform: Transform::from_xyz(
                                    (x * CHUNK_SIZE) as f32,
                                    (y * CHUNK_SIZE) as f32,
                                    (z * CHUNK_SIZE) as f32,
                                ),
                                ..default()
                            },
                        ));
                    }
                }
            }
        }
    }
}

fn generate_chunks(
    perlin_noise: Res<PerlinInfo>,
    cube_handle: Res<CubeHandle>,
    cube_material_handle: Res<CubeMaterialHandle>,
    mut chunks: Query<&mut Chunk>,
    mut commands: Commands,
) {
    for mut chunk in chunks.iter_mut() {
        if chunk.generation_state == GenerationState::NotStarted {
            chunk.generation_state = GenerationState::Generating;
            for x in 0..CHUNK_SIZE {
                let fx = x as f64 + (chunk.x as f64 * CHUNK_SIZE as f64);
                for y in 0..CHUNK_SIZE {
                    let fy = y as f64 + (chunk.y as f64 * CHUNK_SIZE as f64);
                    for z in 0..CHUNK_SIZE {
                        let fz = z as f64 + (chunk.z as f64 * CHUNK_SIZE as f64);
                        let noise_value =
                            perlin_noise.generator.get([fx / 16., fy / 16., fz / 16.]);
                        if noise_value <= 0.0 {
                            // TODO: we're spawning a bunch of cubes. maybe don't?
                            // also we're not cleaning any of them up
                            let bundle = (PbrBundle {
                                mesh: cube_handle.0.clone(),
                                material: cube_material_handle.0.clone(),
                                transform: Transform::from_xyz(fx as f32, fy as f32, fz as f32),
                                ..default()
                            },);
                            commands.spawn(bundle);
                        }
                    }
                }
            }
        }
    }
}

#[derive(PartialEq)]
enum GenerationState {
    NotStarted,
    Generating,
}

#[derive(PartialEq)]
enum LoadState {
    ShouldLoad,
    ShouldUnload,
}

#[derive(Component)]
struct Chunk {
    generation_state: GenerationState,
    load_state: LoadState,
    // blocks: [u8; BLOCKS_PER_CHUNK],
    x: isize,
    y: isize,
    z: isize,
}

fn chunk_to_world_coords(x: isize) -> f32 {
    (x * CHUNK_SIZE) as f32
}

impl Chunk {
    fn center(&self) -> Vec3 {
        Vec3::from((
            chunk_to_world_coords(self.x) + (CHUNK_SIZE as f32 / 2.),
            chunk_to_world_coords(self.y) + (CHUNK_SIZE as f32 / 2.),
            chunk_to_world_coords(self.z) + (CHUNK_SIZE as f32 / 2.),
        ))
    }
}

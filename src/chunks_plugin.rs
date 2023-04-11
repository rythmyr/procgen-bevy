use bevy::{prelude::*, utils::HashSet};

use crate::player::Player;

const CHUNK_SIZE: isize = 16isize;
// const BLOCKS_PER_CHUNK: isize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

const RENDER_DISTANCE_CHUNKS: isize = 8;
const RENDER_DISTANCE_UNITS: isize = RENDER_DISTANCE_CHUNKS * CHUNK_SIZE;

pub struct ChunksPlugin;

impl Plugin for ChunksPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(chunk_load_system);
    }
}

fn chunk_load_system(
    players: Query<&Transform, With<Player>>,
    chunks: Query<(&Chunk, Entity)>, //
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
                // I'm guessing it'll be really useful when we have saving/loading
                commands.entity(entity).despawn();
            }
        }
    }
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

    for player in players.iter() {
        let px = player.translation.x.floor() as isize / CHUNK_SIZE;
        let py = player.translation.y.floor() as isize / CHUNK_SIZE;
        let pz = player.translation.z.floor() as isize / CHUNK_SIZE;

        for x in px - RENDER_DISTANCE_CHUNKS..px + RENDER_DISTANCE_CHUNKS {
            for y in py - RENDER_DISTANCE_CHUNKS..py + RENDER_DISTANCE_CHUNKS {
                for z in pz - RENDER_DISTANCE_CHUNKS..pz + RENDER_DISTANCE_CHUNKS {
                    let chunk = Chunk {
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
                            Chunk {
                                x,
                                y,
                                z,
                                load_state: LoadState::ShouldLoad,
                            },
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

// enum GenerationState {
//     NotStarted,
//     Queued,
//     Generating,
//     Done,
// }

#[derive(PartialEq)]
enum LoadState {
    ShouldLoad,
    // Loaded,
    ShouldUnload,
    // Unloaded,
}

#[derive(Component)]
struct Chunk {
    // generation_state: GenerationState,
    load_state: LoadState,
    // blocks: [u8; BLOCKS_PER_CHUNK],
    x: isize,
    y: isize,
    z: isize,
}

fn chunk_to_world_coords(x: isize) -> f32 {
    (x * CHUNK_SIZE) as f32
}

// fn world_to_chunk_coords(x: f32) -> isize {
//     (x / CHUNK_SIZE as f32) as isize
// }

impl Chunk {
    fn center(&self) -> Vec3 {
        Vec3::from((
            chunk_to_world_coords(self.x) + (CHUNK_SIZE as f32 / 2.),
            chunk_to_world_coords(self.y) + (CHUNK_SIZE as f32 / 2.),
            chunk_to_world_coords(self.z) + (CHUNK_SIZE as f32 / 2.),
        ))
    }
}

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
    mut chunks: Query<(&mut Chunk, Entity)>, //
    mut commands: Commands,
) {
    let mut total_chunks_unloaded = 0;
    let mut existing_chunks = HashSet::<(isize, isize, isize)>::new();
    for tuple in chunks.iter_mut() {
        let (mut chunk, entity) = tuple;
        existing_chunks.insert((chunk.x, chunk.y, chunk.z));
        for player in players.iter() {
            if (chunk.center() - player.translation).length_squared()
                >= (RENDER_DISTANCE_UNITS * RENDER_DISTANCE_UNITS) as f32
                && chunk.load_state != LoadState::ShouldUnload
            {
                chunk.load_state = LoadState::ShouldUnload;
                total_chunks_unloaded += 1;
                commands.entity(entity).despawn();
            }
        }
    }

    let mut total_chunks_added = 0;
    let mut total_chunks_skipped = 0;
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
                        total_chunks_added += 1;
                        commands.spawn(Chunk {
                            x,
                            y,
                            z,
                            load_state: LoadState::ShouldLoad,
                        });
                    } else {
                        total_chunks_skipped += 1;
                    }
                }
            }
        }
    }
    if total_chunks_added != 0 {
        println!(
            "{}, {}, -{}",
            total_chunks_added, total_chunks_skipped, total_chunks_unloaded
        );
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

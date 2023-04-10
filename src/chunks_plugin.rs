use bevy::prelude::*;

use crate::player::Player;

const CHUNK_SIZE: usize = 16usize;
// const BLOCKS_PER_CHUNK: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

const RENDER_DISTANCE_CHUNKS: usize = 16;
const RENDER_DISTANCE_UNITS: usize = RENDER_DISTANCE_CHUNKS * CHUNK_SIZE;

pub struct ChunksPlugin;

impl Plugin for ChunksPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(chunk_load_system)
            .add_system(chunk_unload_system);
    }
}

fn chunk_load_system(
    players: Query<&Transform, With<Player>>,
    mut chunks: Query<&mut Chunk>, //
) {
    // TODO: need to remove chunks that need unloaded
    // TODO: need to add chunks that need loaded
    for mut chunk in chunks.iter_mut() {
        for player in players.iter() {
            if (chunk.center() - player.translation).length_squared()
                >= (RENDER_DISTANCE_UNITS * RENDER_DISTANCE_UNITS) as f32
            {
                chunk.load_state = LoadState::ShouldUnload;
            }
        }
    }
}

pub fn chunk_unload_system() {
    //
}

// enum GenerationState {
//     NotStarted,
//     Queued,
//     Generating,
//     Done,
// }

enum LoadState {
    // ShouldLoad,
    // Loaded,
    ShouldUnload,
    // Unloaded,
}

#[derive(Component)]
struct Chunk {
    // generation_state: GenerationState,
    load_state: LoadState,
    // blocks: [u8; BLOCKS_PER_CHUNK],
    x: i64,
    y: i64,
    z: i64,
}

fn chunk_to_world_coords(x: i64) -> f32 {
    (x * CHUNK_SIZE as i64 + (CHUNK_SIZE as i64 / 2)) as f32
}

impl Chunk {
    fn center(&self) -> Vec3 {
        Vec3::from((
            chunk_to_world_coords(self.x),
            chunk_to_world_coords(self.y),
            chunk_to_world_coords(self.z),
        ))
    }
}

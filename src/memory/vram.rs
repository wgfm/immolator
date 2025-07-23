pub struct VRam<'mem>(pub &'mem [u8]);

const TILE_SIZE: usize = 16;
const TILES_PER_BLOCK: usize = 128;
const BLOCK_SIZE: usize = TILE_SIZE * TILES_PER_BLOCK;
const TILEMAP_SIZE: usize = 1024;

pub fn as_vram(slice: &[u8]) -> VRam {
    VRam(slice)
}

impl<'mem> VRam<'mem> {
    pub fn block1(&self) -> Block {
        Block(&self.0[0..BLOCK_SIZE])
    }

    pub fn block2(&self) -> Block {
        Block(&self.0[BLOCK_SIZE..2*BLOCK_SIZE])
    }

    pub fn block3(&self) -> Block {
        Block(&self.0[2*BLOCK_SIZE..3*BLOCK_SIZE])
    }

    pub fn tilemap_0(&self) -> Tilemap {
        Tilemap(&self.0[3*BLOCK_SIZE..3*BLOCK_SIZE+TILEMAP_SIZE])
    }

    pub fn tilemap_1(&self) -> Tilemap {
        Tilemap(&self.0[3*BLOCK_SIZE+TILEMAP_SIZE..3*BLOCK_SIZE+2*TILEMAP_SIZE])
    }
}

struct Block<'mem>(&'mem [u8]);

struct Tilemap<'mem>(&'mem [u8]);


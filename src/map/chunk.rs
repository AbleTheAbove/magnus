// Make tile types
pub struct TileStack {
    ground: String,
}

pub struct Chunk {
    data: TileStack,
}

// 128x128 chunks would be good
// with 9 loaded at a time?

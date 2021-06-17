use crate::proto_gen;

pub use proto_gen::storage;
pub use proto_gen::storage::*;
pub use proto_gen::storage::{
    client_to_chunk_client::ClientToChunkClient,
    client_to_chunk_server::{
        ClientToChunk,
        ClientToChunkServer
    }
};
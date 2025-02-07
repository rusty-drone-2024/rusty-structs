mod message;
mod serialization;

pub use message::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wg_2024::network::NodeId;

/// Identifier of a file or media.
pub type Link = String;
/// Media data (could be extended e.g. with MIME type).
pub type Media = Vec<u8>;

/// Allowing servers to be identified throughout different topologies, where they get assigned different NodeIds.
pub type ServerUUID = u64;

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServerType {
    Chat,
    Text(ServerUUID),
    Media(ServerUUID),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileWithData {
    pub file: String,
    // For each link which server to get the file/media from.
    pub related_data: HashMap<Link, ServerUUID>,
}


use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Audio,
    Automation,
    Notes,
    Video,
    Markers,
    Tracks,
}

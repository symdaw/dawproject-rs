use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FileReference {
    #[serde(rename = "@path")]
    pub path: String,
    #[serde(rename = "@external")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
}

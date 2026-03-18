use serde::{Deserialize, Serialize};

use super::{note::Note, time_unit::TimeUnit};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Notes {
    // Extends timeline
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "@name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>, // attribute
    #[serde(rename = "@color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>, // att
    #[serde(rename = "@comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>, // att
    #[serde(rename = "@track")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<String>,
    #[serde(rename = "@timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub time_unit: Option<TimeUnit>,

    #[serde(rename = "$value", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes_sequence: Option<Vec<Note>>,
}

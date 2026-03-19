use serde::{Deserialize, Serialize};

use super::{note::Note, time_unit::TimeUnit};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum NoteEnum {
    #[serde(rename = "Note")]
    Note(Note),
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Notes {
    // Extends Timeline
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "@name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "@comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "@track")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<String>,
    #[serde(rename = "@timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub time_unit: Option<TimeUnit>,
    #[serde(rename = "$value", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes_sequence: Option<Vec<NoteEnum>>,
}

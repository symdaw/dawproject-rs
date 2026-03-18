use {
    super::super::file_reference::FileReference,
    serde::{Deserialize, Serialize},
};

use super::time_unit::TimeUnit;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Video {
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
    #[serde(rename = "File")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_sequence: Option<Vec<FileReference>>,
    #[serde(rename = "@duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(rename = "@algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(rename = "@channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<i32>,
    #[serde(rename = "@sampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i32>,
}


use super::{clip::Clip, time_unit::TimeUnit};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClipSlot {
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
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clips: Option<Vec<Clip>>,
    #[serde(rename = "@hasStop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_stop: Option<bool>,
}

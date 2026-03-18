use serde::{Deserialize, Serialize};

use super::{
    audio::Audio, clip_slot::ClipSlot, clips::Clips, markers::Markers, notes::Notes,
    points::Points, time_unit::TimeUnit, timeline::TimeLine, video::Video, warps::Warps,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ArrangementTypeChoiceEnum {
    Timeline(TimeLine),
    Lanes(Lanes),
    Notes(Notes),
    Clips(Clips),
    ClipSlot(ClipSlot),
    Markers(Markers),
    Warps(Warps),
    Audio(Audio),
    Video(Video),
    Points(Points),
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LanesSequence {
    #[serde(rename = "$value", default)]
    pub lanes_sequence: Option<LanesSequenceChoice>,
}

type LanesSequenceChoice = Vec<ArrangementTypeChoiceEnum>;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Lanes {
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
    pub lanes_sequence: Option<LanesSequenceChoice>,
}

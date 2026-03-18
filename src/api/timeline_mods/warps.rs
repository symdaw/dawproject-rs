use super::{
    audio::Audio, clip_slot::ClipSlot, clips::Clips, lanes::Lanes, markers::Markers, notes::Notes,
    points::Points, time_unit::TimeUnit, timeline::TimeLine, video::Video, warp::Warp,
    UpcastTimeline,
};

use {
    
        serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum WarpsSequenceEnum {
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
    Warp(Warp),
}

type WarpsSequence = Vec<WarpsSequenceEnum>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Warps {
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
    #[serde(rename = "@contentTimeUnit")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub content_time_unit: Option<TimeUnit>,
    #[serde(rename = "$value", default)]
    pub warps_sequence: Option<WarpsSequence>,
}



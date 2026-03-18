use {
    super::timeline_mods::{
        audio::Audio, clip_slot::ClipSlot, clips::Clips, lanes::Lanes, markers::Markers,
        notes::Notes, points::Points, timeline::TimeLine, video::Video, warps::Warps,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
enum SceneSequenceEnum {
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Scene {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@color")]
    pub color: Option<String>,
    #[serde(rename = "@comment")]
    pub comment: Option<String>,
    #[serde(rename = "$value", default)]
    pub content: Option<SceneSequenceEnum>,
}

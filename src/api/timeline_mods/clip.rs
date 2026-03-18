use {
    super::{lanes::ArrangementTypeChoiceEnum, time_unit::TimeUnit, timeline::TimeLine},
    serde::{Deserialize, Serialize},
};
type ClipSequenceChoice = Vec<ArrangementTypeChoiceEnum>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TimeLineEnum {
    TimeLine(TimeLine),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Clip {
    #[serde(rename = "@name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "@comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "$value", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes_sequence_choice: Option<ClipSequenceChoice>,
    #[serde(rename = "@time")]
    pub time: f64,
    #[serde(rename = "@duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(rename = "@contentTimeUnit")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub content_time_unit: Option<TimeUnit>,
    #[serde(rename = "@playStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_start: Option<f64>,
    #[serde(rename = "@playStop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_stop: Option<f64>,
    #[serde(rename = "@loopStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_start: Option<f64>,
    #[serde(rename = "@loopEnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_end: Option<f64>,
    #[serde(rename = "@fadeTimeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_time_unit: Option<TimeUnit>,
    #[serde(rename = "@fadeInTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in_time: Option<f64>,
    #[serde(rename = "@fadeOutTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_out_time: Option<f64>,
    #[serde(rename = "@reference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}
